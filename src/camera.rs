use winit::{
    dpi::PhysicalSize,
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use super::types;

/// Describes a how the camera is moving
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    /// All the settings
    settings: CameraSettings,
    /// The movement keys: d, a, w, s
    active_move: [bool; 4],
    /// The zoom keys: q, e
    active_zoom: [bool; 2],
    /// If true then the camera transforms are speed up
    boost: bool,
    /// True if any button is pressed and the camera needs to be updated
    active: bool,
    /// The current transform
    transform: types::Transform2D,
    /// The transform to make the aspect ratio correct
    transform_aspect: types::Transform2D,
    /// The transform to apply to the current transform every frame
    transform_update: types::Transform2D,
}

impl Camera {
    /// Creates a new camera
    ///
    /// # Parameters
    ///
    /// settings: All the settings for the camera
    ///
    /// transform: The initial transform to use
    pub fn new(settings: CameraSettings, transform: types::Transform2D) -> Self {
        Self {
            active_move: [false; 4],
            active_zoom: [false; 2],
            boost: false,
            active: false,
            settings,
            transform,
            transform_aspect: Self::size_to_aspect(&PhysicalSize {
                width: 1,
                height: 1,
            }),
            transform_update: types::Transform2D::identity(),
        }
    }

    /// Retrieves a reference to the settings
    pub fn get_settings(&self) -> &CameraSettings {
        return &self.settings;
    }

    /// Sets new settings for the camera
    ///
    /// # Parameters
    ///
    /// settings: The new settings to set
    pub fn set_settings(&mut self, settings: CameraSettings) {
        self.settings = settings;
        self.reload_transform();
    }

    /// Attempts to use a key press from a key event, if the key press is used,
    /// it returns true, if it is ignored, it returns false
    ///
    /// # Parameters
    ///
    /// event: The key event to handle
    pub fn apply_key(&mut self, event: &KeyEvent) -> bool {
        // Stop if it is a repeat press
        if event.repeat {
            return false;
        }

        let active = match event.state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };

        match event.physical_key {
            PhysicalKey::Unidentified(_) => return false,
            PhysicalKey::Code(code) => match code {
                KeyCode::KeyD => self.active_move[0] = active,
                KeyCode::KeyA => self.active_move[1] = active,
                KeyCode::KeyW => self.active_move[2] = active,
                KeyCode::KeyS => self.active_move[3] = active,
                KeyCode::KeyQ => self.active_zoom[0] = active,
                KeyCode::KeyE => self.active_zoom[1] = active,
                KeyCode::ShiftLeft => self.boost = active,
                _ => return false,
            },
        };

        // Reload the update transform
        self.reload_transform();

        return true;
    }

    /// Reset all of the input such that all of it is turned off
    pub fn reset_keys(&mut self) {
        self.active_move.iter_mut().for_each(|val| *val = false);
        self.active_zoom.iter_mut().for_each(|val| *val = false);
        self.boost = false;
        self.reload_transform();
    }

    /// Recalculates the aspect transform after resizing
    ///
    /// # Parameters
    ///
    /// size: THe new size of the window
    pub fn resize(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        self.transform_aspect = Self::size_to_aspect(size);
    }

    /// Retrieves the transform
    pub fn get_transform(&self) -> types::Transform2D {
        return self.transform_aspect * self.transform;
    }

    pub fn get_world_transform(&self) -> &types::Transform2D {
        return &self.transform;
    }

    /// Sets a new transform
    ///
    /// # Parameters
    ///
    /// transform: The new transform to set
    pub fn set_transform(&mut self, transform: types::Transform2D) {
        self.transform = transform;

        // Correct zoom level
        self.enforce_limits();
    }

    /// Update the transform using the current input, should be run once per frame
    ///
    /// Returns true if the transform has updated
    pub fn update_transform(&mut self) -> bool {
        if !self.active {
            return false;
        }

        self.transform = self.transform_update * self.transform;

        // Correct zoom level
        self.enforce_limits();

        return true;
    }

    /// Reload the transform_update for when the input has changed
    fn reload_transform(&mut self) {
        // Check if it is active
        self.active = self.active_move.iter().any(|&x| x) || self.active_zoom.iter().any(|&x| x);

        if !self.active {
            return;
        }

        // Calculate the movement velocity
        let move_speed = self.settings.speed_move / self.settings.framerate
            * if self.boost {
                self.settings.boost_factor
            } else {
                1.0
            };
        const KEY_DIRECTION: [types::Point; 4] = [
            types::Point { x: 1.0, y: 0.0 },
            types::Point { x: -1.0, y: 0.0 },
            types::Point { x: 0.0, y: 1.0 },
            types::Point { x: 0.0, y: -1.0 },
        ];
        let mut move_dir = self
            .active_move
            .iter()
            .zip(KEY_DIRECTION.iter())
            .filter_map(|(&active, dir)| if active { Some(dir) } else { None })
            .fold(types::Point::new(0.0, 0.0), |prev, next| prev + next);
        if move_dir.x != 0.0 || move_dir.y != 0.0 {
            move_dir = move_dir * move_speed / move_dir.norm();
        }

        // Calculate the zoom velocity
        let zoom_val = 1.0
            + self.settings.speed_zoom / self.settings.framerate
                * if self.boost {
                    self.settings.boost_factor
                } else {
                    1.0
                };
        let key_zoom = [1.0 / zoom_val, zoom_val];
        let zoom_dir = self
            .active_zoom
            .iter()
            .zip(key_zoom.iter())
            .filter_map(|(&active, zoom)| if active { Some(zoom) } else { None })
            .fold(1.0, |prev, next| prev * next);

        // Combine all of the transforms
        let transform_move = types::Transform2D::translate(&(-move_dir));
        let transform_zoom = types::Transform2D::scale(&types::Point::new(zoom_dir, zoom_dir));
        self.transform_update = transform_move * transform_zoom;
    }

    /// Make sure all limits are held
    pub fn enforce_limits(&mut self) {
        // Enforce zoom limits
        let zoom_level = self.transform.get_scaling_x();
        let zoom_clamped =
            zoom_level.clamp(self.settings.zoom_limits.0, self.settings.zoom_limits.1);
        if zoom_level != zoom_clamped && zoom_level.is_normal() {
            let zoom_correction = zoom_clamped / zoom_level;
            self.transform = types::Transform2D::scale(&types::Point {
                x: zoom_correction,
                y: zoom_correction,
            }) * self.transform;
        }

        // Wrap position
        self.transform.center.x %= self.settings.map_width * zoom_clamped;
        if self.transform.center.x > 0.0 {
            self.transform.center.x -= self.settings.map_width * zoom_clamped;
        }
    }

    /// Converts a size to an aspect transform
    ///
    /// # Parameters
    ///
    /// size: The size of the window
    fn size_to_aspect(size: &PhysicalSize<u32>) -> types::Transform2D {
        let ratio = if size.height == 0 {
            1.0
        } else {
            (size.width as f64) / (size.height as f64)
        };

        return types::Transform2D::scale(&types::Point::new(1.0, ratio));
    }
}

/// All settings for a camera
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraSettings {
    /// The speed of movement
    pub speed_move: f64,
    /// The speed of zooming
    pub speed_zoom: f64,
    /// The speed factor when boosting
    pub boost_factor: f64,
    /// The framerate of the program, this is how many times a second the transform should be updated
    pub framerate: f64,
    /// The minimum and maximum allowed zoom
    pub zoom_limits: (f64, f64),
    /// The width of the map used for wrapping
    pub map_width: f64,
}

impl CameraSettings {
    /// Creates camera settings with default values
    pub fn default() -> Self {
        return Self {
            speed_move: 4.0,
            speed_zoom: 1.2,
            boost_factor: 2.0,
            framerate: 60.0,
            zoom_limits: (0.0, f64::INFINITY),
            map_width: f64::MAX,
        };
    }

    /// Changes the movement speed and returns the updated object
    ///
    /// # Parameters
    ///
    /// speed: The new movement speed
    pub fn with_speed_move(mut self, speed: f64) -> Self {
        self.speed_move = speed;
        return self;
    }

    /// Changes the zoom speed and returns the updated object
    ///
    /// # Parameters
    ///
    /// speed: The new zoom speed
    pub fn with_speed_zoom(mut self, speed: f64) -> Self {
        self.speed_zoom = speed;
        return self;
    }

    /// Changes the boost factor and returns the updated object
    ///
    /// # Parameters
    ///
    /// factor: The new boost factor
    pub fn with_boost_factor(mut self, factor: f64) -> Self {
        self.boost_factor = factor;
        return self;
    }

    /// Changes the framerate and returns the updated object
    ///
    /// # Parameters
    ///
    /// framerate: The new framerate
    pub fn with_framerate(mut self, framerate: f64) -> Self {
        self.framerate = framerate;
        return self;
    }

    /// Changes the zoom limits and returns the updated object
    ///
    /// # Parameters
    ///
    /// zoom_limits: The minimum and maximum zoom
    pub fn with_zoom_limits(mut self, zoom_limits: (f64, f64)) -> Self {
        self.zoom_limits = zoom_limits;
        return self;
    }

    /// Changes the map width and returns the updated object
    ///
    /// # Parameters
    ///
    /// width: The map width
    pub fn with_map_width(mut self, width: f64) -> Self {
        self.map_width = width;
        return self;
    }
}
