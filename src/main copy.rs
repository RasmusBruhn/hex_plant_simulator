use std::env;

use winit::dpi::PhysicalSize;

pub mod application;
pub mod camera;
pub mod constants;
pub mod function;
pub mod graphics;
pub mod linear_algebra;
pub mod map;
pub mod operators;
pub mod render;
pub mod solver;
pub mod types;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Get crate data
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    // Setup the camera
    let camera_transform = types::Transform2D::scale(&types::Point::new(1.0, 1.0));
    let camera_settings = camera::CameraSettings::default()
        .with_framerate(constants::FRAMERATE)
        .with_speed_move(constants::CAMERA_MOVE_SPEED)
        .with_speed_zoom(constants::CAMERA_ZOOM_SPEED)
        .with_boost_factor(constants::CAMERA_BOOST_FACTOR);
    let camera = camera::Camera::new(camera_settings, camera_transform);

    // Set window settings
    let name = format!("{crate_name} v{crate_version}");
    let size = PhysicalSize::new(500, 500);
    let color_background = constants::COLOR_BACKGROUND;
    let graphics_settings = graphics::Settings { color_background };
    let settings_window = application::WindowSettingsInput {
        name,
        size,
        graphics_settings,
    };

    // Setup the shader settings
    let color_map = graphics::ColorMap {
        saturated: constants::COLOR_MAP_SATURATED,
        empty: constants::COLOR_MAP_EMPTY,
    };
    let settings_shader = application::ShaderSettingsInput { color_map };

    // Setup the viewer settings
    let framerate = constants::FRAMERATE;
    let settings_viewer = application::ViewerSettingsInput { framerate };

    // Setup the external force
    let coordinate_x = function::coordinate(
        &constants::GRID_SIZE,
        constants::GRID_DELTA,
        operators::Direction::X,
    );
    let coordinate_y = function::coordinate(
        &constants::GRID_SIZE,
        constants::GRID_DELTA,
        operators::Direction::Y,
    );
    let coordinate = [coordinate_x, coordinate_y];
    let force_gravity_x = function::function(&coordinate, &|_| {
        return 0.0;
    });
    let force_gravity_y = function::function(&coordinate, &|_| {
        return -constants::MAP_GRAVITY;
    });
    let force_gravity = operators::VectorField {
        x: force_gravity_x,
        y: force_gravity_y,
    };
    let force_external_x = function::function(&coordinate, &|coords| {
        return 10.0
            * (-(coords[0] * coords[0] + coords[1] * coords[1]) / (2.0 * 0.25 * 0.25)).exp();
    });
    let force_external_y = function::function(&coordinate, &|_| {
        return 0.0;
    });
    let force_external = operators::VectorField {
        x: force_external_x,
        y: force_external_y,
    };

    // Set the starting current
    let current = operators::VectorField {
        x: function::constant(&constants::GRID_SIZE, 0.0),
        y: function::constant(&constants::GRID_SIZE, 0.0),
    };

    // Setup the solver
    let solver = solver::ForwardEuler::new(
        &constants::GRID_SIZE,
        constants::GRID_DELTA,
        constants::SOLVER_TIME_STEP,
        constants::MAP_VISCOSITY,
        constants::SOLVER_THRESHOLD,
        constants::SOLVER_MAX_ITERATIONS,
        current,
        |_| &force_gravity + &(&force_external * 1.0),
    )
    .unwrap();

    // Construct the map
    let map_data = map::Map::new(solver);

    // Setup the main loop
    let mut main_loop = application::MainLoop::new(
        map_data,
        camera,
        settings_window,
        settings_shader,
        settings_viewer,
    );

    // Run the application
    application::run(&mut main_loop);
}
