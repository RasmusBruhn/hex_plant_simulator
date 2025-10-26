use crate::{camera, graphics, map, render, types};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    dpi::{PhysicalPosition, PhysicalSize},
    event::{DeviceId, ElementState, KeyEvent, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

/// Runs the application
pub fn run(main_loop: &mut MainLoop) {
    // Setup logging
    env_logger::init();

    // Create the event loop
    let event_loop = match EventLoop::new() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Unable to create event loop: {:?}", error);
            return;
        }
    };

    if let Err(error) = event_loop.run_app(main_loop) {
        eprintln!("An error occured in the main loop: {:?}", error);
        return;
    }
}

/// Controls the main game loop of the application
pub struct MainLoop {
    /// The currently opened window of the application
    window: Option<RenderedWindow>,
    /// The map of tiles
    map: map::Map,
    /// The camera for controlling what is displayed
    camera: camera::Camera,
    /// All the settings for creating and displaying a window
    settings_window: WindowSettings,
    /// All settings for the shader
    settings_shader: ShaderSettings,
    /// All settings for viewing the application
    settings_viewer: ViewerSettings,
    /// If true, then the image must be updated before displaying
    update_image: bool,
    /// If true, then the simulation must be iterated once
    iterate_simulation: bool,
    /// If true then the simulation is constantly running
    run_simulation: bool,
    /// If true then the simulation must be redrawn next frame
    redraw_simulation: bool,
    /// The next time the frame has increased
    next_frame_time: Instant,
    /// The next time the simulation must step
    next_sim_time: Instant,
}

impl MainLoop {
    /// Creates a new main loop with the supplied settings
    ///
    /// # Parameters
    ///
    /// map: The full map
    ///
    /// camera: The camera for controlling what is displayed
    ///
    /// settings_window: All the settings for creating and displaying a window
    ///
    /// settings_shader: All settings for the shader
    ///
    /// settings_viewer: All settings for viewing the application
    pub fn new(
        map: map::Map,
        mut camera: camera::Camera,
        settings_window: WindowSettingsInput,
        settings_shader: ShaderSettingsInput,
        settings_viewer: ViewerSettingsInput,
    ) -> Self {
        // Set the width of the map in the camera
        camera.get_settings_mut().get_mut().map_width = map.get_size().w as f64;

        // Create the window settings
        let settings_window = WindowSettings {
            name: settings_window.name,
            size: settings_window.size,
            graphics_settings: settings_window.graphics_settings,
        };

        // Create the shader settings
        let settings_shader = ShaderSettings {
            input_settings: settings_shader,
            grid_layout: map.get_grid_layout(),
        };

        // Create the viewer settings
        let home_view = types::View::new(
            types::Point::new(
                (map.get_size().w as f64) * 0.5,
                (map.get_size().h as f64) * 0.5,
            ),
            types::Size::new(map.get_size().w as f64, map.get_size().h as f64),
        );
        let settings_viewer = ViewerSettings {
            input_settings: settings_viewer,
            home_view,
        };

        return Self {
            window: None,
            map,
            camera,
            settings_window,
            settings_shader,
            settings_viewer,
            update_image: false,
            iterate_simulation: false,
            run_simulation: false,
            redraw_simulation: false,
            next_frame_time: Instant::now(),
            next_sim_time: Instant::now(),
        };
    }

    /// Homes the view
    fn home(&mut self) {
        let height = if self.settings_window.size.height == 0 {
            1.0
        } else {
            (self.settings_window.size.height as f64) / (self.settings_window.size.height as f64)
        };

        // Get the scales in x and y such that the view is exactly in the screen
        let scale_x = if self.settings_viewer.home_view.get_size().get_w() == 0.0 {
            0.0
        } else {
            1.0 / self.settings_viewer.home_view.get_size().get_w()
        };
        let scale_y = if self.settings_viewer.home_view.get_size().get_h() == 0.0 {
            0.0
        } else {
            height / self.settings_viewer.home_view.get_size().get_h()
        };

        // Find the scale such that both x and y is in the screen
        let scale = 2.0 * scale_x.min(scale_y);

        // Create the transform
        let transform = types::Transform2D::scale(&types::Point::new(scale, scale))
            * types::Transform2D::translate(&(-self.settings_viewer.home_view.get_center()));

        // Reset the camera and set the new transform
        self.camera.reset_keys();
        self.camera.set_transform(transform);
    }

    /// Updates the color map for the background based on the set settings
    fn set_color_map_background(&self) {
        let window = match &self.window {
            Some(window) => window,
            None => {
                eprintln!("Cannot get window because it is not initialized");
                return;
            }
        };

        window.get_graphics_state().set_color_map(
            &window.render_state,
            &graphics::InstanceType::GridBackground(
                window.graphics_state.get_settings().mode_tiles_background,
            ),
            &self
                .settings_shader
                .input_settings
                .color_map_tiles_background[window
                .get_graphics_state()
                .get_settings()
                .mode_tiles_background
                .id()],
        );
    }

    /// Changes the display mode for the background
    ///
    /// # Parameters
    ///
    /// mode: The way to change the display mode
    fn change_mode_background(&mut self, mode: &ChangeMode) {
        {
            let window = match &mut self.window {
                Some(window) => window,
                None => {
                    eprintln!("Cannot get window because it is not initialized");
                    return;
                }
            };

            // Set the display mode
            window
                .get_graphics_state_mut()
                .get_settings_mut()
                .mode_tiles_background = match mode {
                ChangeMode::Next => window
                    .get_graphics_state()
                    .get_settings()
                    .mode_tiles_background
                    .next(),
                ChangeMode::Prev => window
                    .get_graphics_state()
                    .get_settings()
                    .mode_tiles_background
                    .prev(),
                ChangeMode::Id(id) => map::DataModeBackground::from_id(*id),
            };
            window.get_window().request_redraw();

            // Update the map
            window
                .get_graphics_state()
                .update_map(&window.get_render_state(), &self.map);
        }

        // Set the new color map
        self.set_color_map_background();
        self.request_redraw();
    }

    /// Requests a redraw to the system
    fn request_redraw(&self) {
        let window = match &self.window {
            Some(window) => window,
            None => {
                eprintln!("Cannot get window because it is not initialized");
                return;
            }
        };

        window.get_window().request_redraw();
    }

    /// Handles the initialization of the game loop
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop to handle
    fn game_loop_init(&mut self, event_loop: &ActiveEventLoop) {
        // Start the event loop
        event_loop.set_control_flow(ControlFlow::Poll);

        // Set the size of the camera
        self.camera.resize(&self.settings_window.size);

        // Home the view
        self.home();
    }

    /// Handles the iteration of the game loop
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop to handle
    ///
    /// requested_resume: The time requested to resume
    fn game_loop_iteration(&mut self, event_loop: &ActiveEventLoop, requested_resume: Instant) {
        // Update the time, make sure we do not get a backlog by skipping if we should wait until before now
        let now_time = Instant::now();
        let (new_time_frame, forward_frame) = if now_time < self.next_frame_time {
            (self.next_frame_time, false)
        } else {
            let duration = Duration::from_micros(
                (1e6 / self.settings_viewer.input_settings.framerate).floor() as u64,
            );
            let new_time = requested_resume + duration;
            if new_time < now_time {
                (now_time + duration, true)
            } else {
                (new_time, true)
            }
        };
        let (new_time_sim, forward_sim) = if !self.run_simulation {
            (new_time_frame, false)
        } else if now_time < self.next_sim_time {
            (self.next_sim_time, false)
        } else {
            let duration = Duration::from_micros(
                (1e6 / self.settings_viewer.input_settings.sim_rate).floor() as u64,
            );
            let new_time = requested_resume + duration;
            if new_time < now_time {
                (now_time + duration, true)
            } else {
                (new_time, true)
            }
        };
        event_loop.set_control_flow(ControlFlow::WaitUntil(new_time_frame.min(new_time_sim)));

        // Get the window and id
        let window = match &self.window {
            Some(window) => window,
            None => {
                eprintln!("Cannot process game loop because window is not initialized");
                return;
            }
        };

        // Handle frame iteration
        if forward_frame {
            // Update the camera
            if self.camera.update_transform() {
                window.get_window().request_redraw();
            }
        }

        // Update the simulation
        if self.iterate_simulation || forward_sim {
            self.iterate_simulation = false;
            self.update_image = true;
            self.redraw_simulation = true;
            self.map.step();
        }

        // Request a redraw because of the simulation
        if forward_frame && self.redraw_simulation {
            self.redraw_simulation = false;
            window.get_window().request_redraw();
        }
    }

    /// Handles a window event for the main window
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    ///
    /// event: The event to be handled
    fn main_window_event(&mut self, event_loop: &ActiveEventLoop, event: WindowEvent) {
        // Find the correct event
        match event {
            WindowEvent::CloseRequested => self.main_window_close_request(event_loop),
            WindowEvent::RedrawRequested => self.main_window_redraw_requested(),
            WindowEvent::Resized(size) => self.main_window_resized(size),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => self.main_window_keyboard_input(event_loop, device_id, event, is_synthetic),
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => self.main_window_curser_move(event_loop, device_id, position),
            _ => (),
        }
    }

    /// Run when the main window is to be closed
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    fn main_window_close_request(&self, event_loop: &ActiveEventLoop) {
        // Stop the application
        event_loop.exit();
    }

    /// Run when the main window must be redrawn
    fn main_window_redraw_requested(&mut self) {
        // Get the window and id
        let window = match &self.window {
            Some(window) => window,
            None => {
                eprintln!("Cannot process draw window because window is not initialized");
                return;
            }
        };

        // Update the image
        if self.update_image {
            self.update_image = false;
            window
                .get_graphics_state()
                .update_map(window.get_render_state(), &mut self.map);
        }

        // Get the current view
        let output_texture = match window
            .get_render_state()
            .get_surface()
            .get_current_texture()
        {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Unable to get texture: {:?}", error);
                return;
            }
        };
        let view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Get the transform
        let transform = self.camera.get_transform();
        let transform_pos = transform
            * types::Transform2D::translate(&types::Point {
                x: self.camera.get_settings().map_width,
                y: 0.0,
            });
        let transform_neg = transform
            * types::Transform2D::translate(&types::Point {
                x: -self.camera.get_settings().map_width,
                y: 0.0,
            });

        // Clear the screen
        window.graphics_state.clear(&window.render_state, &view);

        // Render the sun
        window
            .graphics_state
            .render_sun(&window.render_state, &view, &transform_neg);
        window
            .graphics_state
            .render_sun(&window.render_state, &view, &transform_pos);
        window
            .graphics_state
            .render_sun(&window.render_state, &view, &transform);

        // Render the background of the tiles
        window
            .graphics_state
            .render_tiles_background(&window.render_state, &view, &transform_neg);
        window
            .graphics_state
            .render_tiles_background(&window.render_state, &view, &transform_pos);
        window
            .graphics_state
            .render_tiles_background(&window.render_state, &view, &transform);

        // Show to screen
        output_texture.present();
    }

    /// Run when the size of the window has changed
    ///
    /// # Parameters
    ///
    /// size: The new size of the window
    fn main_window_resized(&mut self, size: PhysicalSize<u32>) {
        // Skip if it is zero
        if size.width == 0 || size.height == 0 {
            return;
        }

        // Set the new size
        self.settings_window.size = size;

        // Update the window
        self.window
            .as_mut()
            .expect("Should not happen")
            .get_render_state_mut()
            .resize(size);

        // Update the camera
        self.camera.resize(&size);
    }

    /// Handles any keyboard input like camera movement
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    ///
    /// device_id: The id of the device giving the input
    ///
    /// event: The event to handle
    ///
    /// is_synthetic: True if the event was created by winit
    fn main_window_keyboard_input(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: KeyEvent,
        _is_synthetic: bool,
    ) {
        // Handle camera events, stop if input was captured
        if self.camera.apply_key(&event) {
            return;
        }

        // Handle all non-repeating pressed keys
        let mut update = false;
        if event.state == ElementState::Pressed && !event.repeat {
            match event.physical_key {
                PhysicalKey::Unidentified(_) => (),
                PhysicalKey::Code(code) => match code {
                    KeyCode::KeyH => {
                        // Return to home view
                        self.home();
                        update = true;
                    }
                    KeyCode::Escape => {
                        // Close the application
                        event_loop.exit();
                    }
                    KeyCode::Space => {
                        // Toggle the simulation
                        self.run_simulation = !self.run_simulation;
                    }
                    KeyCode::Digit1 => {
                        // Go to background display mode 0
                        self.change_mode_background(&ChangeMode::Id(0));
                    }
                    KeyCode::Digit2 => {
                        // Go to background display mode 1
                        self.change_mode_background(&ChangeMode::Id(1));
                    }
                    KeyCode::Digit3 => {
                        // Go to background display mode 2
                        self.change_mode_background(&ChangeMode::Id(2));
                    }
                    KeyCode::Digit4 => {
                        // Go to background display mode 3
                        self.change_mode_background(&ChangeMode::Id(3));
                    }
                    KeyCode::Digit5 => {
                        // Go to background display mode 4
                        self.change_mode_background(&ChangeMode::Id(4));
                    }
                    KeyCode::Digit6 => {
                        // Go to background display mode 5
                        self.change_mode_background(&ChangeMode::Id(5));
                    }
                    KeyCode::Digit7 => {
                        // Go to background display mode 6
                        self.change_mode_background(&ChangeMode::Id(6));
                    }
                    KeyCode::Digit8 => {
                        // Go to background display mode 7
                        self.change_mode_background(&ChangeMode::Id(7));
                    }
                    KeyCode::Digit9 => {
                        // Go to background display mode 8
                        self.change_mode_background(&ChangeMode::Id(8));
                    }
                    KeyCode::Digit0 => {
                        // Go to background display mode 9
                        self.change_mode_background(&ChangeMode::Id(9));
                    }
                    _ => (),
                },
            };
        }

        // Handle all repeating key presses
        if event.state == ElementState::Pressed {
            match event.physical_key {
                PhysicalKey::Unidentified(_) => (),
                PhysicalKey::Code(code) => match code {
                    KeyCode::Enter => {
                        // Forward the simulation once
                        self.iterate_simulation = true;
                    }
                    KeyCode::ArrowRight => {
                        // Go to the next background display mode
                        self.change_mode_background(&ChangeMode::Next);
                    }
                    KeyCode::ArrowLeft => {
                        // Go to the previous background display mode
                        self.change_mode_background(&ChangeMode::Prev);
                    }
                    _ => (),
                },
            };
        }

        // Update the graphics
        if update {
            let window = match &self.window {
                Some(window) => window,
                None => {
                    eprintln!("Cannot get window because it is not initialized");
                    return;
                }
            };
            window.get_window().request_redraw();
        }
    }

    fn main_window_curser_move(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        position: PhysicalPosition<f64>,
    ) {
        // Calculate the wgpu position
        let position_wgpu = &types::Point::new(
            (position.x + 0.5) / self.settings_window.size.width as f64,
            1.0 - (position.y + 0.5) / self.settings_window.size.height as f64,
        ) * 2.0
            - &types::Point::new(1.0, 1.0);

        // Get the position in world coordinates
        let _position_world = &self.camera.get_transform().inv() * &position_wgpu;

        // TODO: Add hover functionality
    }
}

impl ApplicationHandler for MainLoop {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Open a new window
        let window_attributes = Window::default_attributes()
            .with_title(&self.settings_window.name)
            .with_inner_size(self.settings_window.size);

        let window = match event_loop.create_window(window_attributes) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Unable to create window: {:?}", error);
                event_loop.exit();
                return;
            }
        };

        // Add a render state
        self.window = match pollster::block_on(RenderedWindow::new(
            window,
            self.settings_window.graphics_settings,
            &mut self.map,
        )) {
            Ok(value) => Some(value),
            Err(error) => {
                eprintln!("Unable to add render state: {:?}", error);
                event_loop.exit();
                self.window = None;
                return;
            }
        };

        // Set the color map for the background
        self.set_color_map_background();

        // Set the grid layout and color map for the sun
        let window = self.window.as_mut().expect("Should never happen");

        window.get_graphics_state().set_color_map(
            &window.render_state,
            &graphics::InstanceType::Sun,
            &self.settings_shader.input_settings.color_map_sun,
        );
        window
            .get_graphics_state()
            .set_grid_layout(&window.render_state, &self.settings_shader.grid_layout);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // Get the window and id
        let window = match &self.window {
            Some(window) => window,
            None => {
                eprintln!("Cannot process events because window is not initialized");
                return;
            }
        };

        // Find the correct window and handle event correspondingly
        if window_id == window.get_window().id() {
            self.main_window_event(event_loop, event);
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => self.game_loop_init(event_loop),
            StartCause::ResumeTimeReached {
                requested_resume, ..
            } => self.game_loop_iteration(event_loop, requested_resume),
            StartCause::Poll => self.game_loop_iteration(event_loop, Instant::now()),
            _ => (),
        };
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // Close the window
        self.window = None;
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // Close the window
        self.window = None;
    }
}

/// Describes how to change the display mode
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ChangeMode {
    /// Change to the next mode
    Next,
    /// Change to the previous mode
    Prev,
    /// Change to a specific mode
    Id(usize),
}

/// All input settings for how to open and display a window
#[derive(Clone, Debug)]
pub struct WindowSettingsInput {
    /// The name of the application
    pub name: String,
    /// The size of the application window
    pub size: PhysicalSize<u32>,
    /// The settings for rendering
    pub graphics_settings: graphics::Settings,
}

/// All settings for how to open and display a window
#[derive(Clone, Debug)]
pub struct WindowSettings {
    /// The name of the application
    pub name: String,
    /// The size of the application window
    pub size: PhysicalSize<u32>,
    /// The settings for rendering
    pub graphics_settings: graphics::Settings,
}

/// All input settings for the shader
#[derive(Clone, Debug)]
pub struct ShaderSettingsInput {
    /// The color maps for the background of the tiles
    pub color_map_tiles_background: [graphics::ColorMap; map::DataModeBackground::COUNT],
    /// The color map for the sun
    pub color_map_sun: graphics::ColorMap,
}

/// All settings for the shader
#[derive(Clone, Debug)]
pub struct ShaderSettings {
    /// All input settings
    pub input_settings: ShaderSettingsInput,
    /// The layout of the grid for displaying
    pub grid_layout: map::GridLayout,
}

/// All input settings how to view the app
#[derive(Clone, Debug)]
pub struct ViewerSettingsInput {
    /// The framerate of the application
    pub framerate: f64,
    /// The number of simulation steps per second
    pub sim_rate: f64,
    /// The multiplier when speeding up or slowing down the simulation
    pub sim_rate_mod: f64,
}

/// All settings how to view the app
#[derive(Clone, Debug)]
pub struct ViewerSettings {
    /// The input settings
    pub input_settings: ViewerSettingsInput,
    /// The home view for the camera
    pub home_view: types::View,
}

/// A window with an assosciated render state
pub struct RenderedWindow {
    /// The window, it must be in an Arc because it is shared with the render state
    window: Arc<Window>,
    /// The render state to render onto the window
    render_state: render::RenderState,
    /// The graphics state used for rendering
    graphics_state: graphics::State,
}

impl RenderedWindow {
    /// Constructs a new rendered window
    ///
    /// # Parameters
    ///
    /// window: The window to add a render state to
    ///
    /// graphics_settings: The settings for the graphics
    ///
    /// map: The map to render
    pub async fn new(
        window: Window,
        graphics_settings: graphics::Settings,
        map: &map::Map,
    ) -> Result<Self, render::NewRenderStateError> {
        let window = Arc::new(window);
        let render_state = render::RenderState::new(&window).await?;
        let graphics_state = graphics::State::new(&render_state, graphics_settings, map);

        return Ok(Self {
            window,
            render_state,
            graphics_state,
        });
    }

    /// Retrieves a reference to the render state
    pub fn get_render_state(&self) -> &render::RenderState {
        return &self.render_state;
    }

    /// Retrieves a mutable reference to the render state
    pub fn get_render_state_mut(&mut self) -> &mut render::RenderState {
        return &mut self.render_state;
    }

    /// Retrieves a reference to the graphics state
    pub fn get_graphics_state(&self) -> &graphics::State {
        return &self.graphics_state;
    }

    /// Retrieves a mutable reference to the graphics state
    pub fn get_graphics_state_mut(&mut self) -> &mut graphics::State {
        return &mut self.graphics_state;
    }

    /// Retrieves a reference to the window
    pub fn get_window(&self) -> &Window {
        return &self.window;
    }
}
