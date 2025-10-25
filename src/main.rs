use std::env;

use winit::dpi::PhysicalSize;

pub mod application;
pub mod camera;
pub mod constants;
pub mod graphics;
pub mod map;
pub mod render;
pub mod types;

fn main() {
    unsafe { env::set_var("RUST_BACKTRACE", "1") };

    // Get crate data
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    // Setup the camera
    let camera_transform = types::Transform2D::scale(&types::Point::new(1.0, 1.0));
    let camera_settings = camera::CameraSettings::default()
        .with_framerate(constants::FRAMERATE)
        .with_speed_move(constants::CAMERA_MOVE_SPEED)
        .with_speed_zoom(constants::CAMERA_ZOOM_SPEED)
        .with_boost_factor(constants::CAMERA_BOOST_FACTOR)
        .with_zoom_limits(constants::CAMERA_ZOOM_LIMITS);
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
    let color_map_tiles_background = graphics::ColorMap {
        saturated: constants::COLOR_MAP_TILES_BACKGROUND_SATURATED,
        empty: constants::COLOR_MAP_TILES_BACKGROUND_EMPTY,
    };
    let color_map_sun = graphics::ColorMap {
        saturated: constants::COLOR_MAP_SUN_SATURATED,
        empty: constants::COLOR_MAP_SUN_EMPTY,
    };
    let settings_shader = application::ShaderSettingsInput {
        color_map_tiles_background,
        color_map_sun,
    };

    // Setup the viewer settings
    let framerate = constants::FRAMERATE;
    let settings_viewer = application::ViewerSettingsInput { framerate };

    // Construct the map
    let map_data = map::Map::new_gradient_x(constants::GRID_SIZE);

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
