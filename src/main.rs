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

    // Setup the shader settings
    let color_map_sun: Box<dyn types::ColorMap> = Box::new(constants::COLOR_MAP_LIGHT);
    let color_map_background_transparency: Box<dyn types::ColorMap> =
        Box::new(constants::COLOR_MAP_TRANSPARENCY);
    let color_map_background_light: Box<dyn types::ColorMap> = Box::new(constants::COLOR_MAP_LIGHT);
    let color_maps_background = map::DataModeBackground::new_color_map_collection(
        color_map_background_light,
        color_map_background_transparency,
    );

    // Set window settings
    let name = format!("{crate_name} v{crate_version}");
    let size = PhysicalSize::new(500, 500);
    let color_background = constants::COLOR_BACKGROUND;
    let mode_background = constants::COLOR_MODE_BACKGROUND;
    let active_color_maps =
        graphics::InstanceType::new_color_map_collection(color_map_sun, color_maps_background);
    let graphics_settings = graphics::Settings {
        color_clear: color_background,
        mode_background,
        color_maps: active_color_maps,
    };
    let settings_window = application::WindowSettingsInput {
        name,
        size,
        graphics_settings,
    };

    // Setup shader settings
    let settings_shader = application::ShaderSettingsInput {};

    // Setup the viewer settings
    let framerate = constants::FRAMERATE;
    let sim_rate = constants::SIM_RATE;
    let sim_rate_mod = constants::SIM_RATE_MODIFIER;
    let settings_viewer = application::ViewerSettingsInput {
        framerate,
        sim_rate,
        sim_rate_mod,
    };

    // Construct the map
    let map_transparency_settings =
        map::settings::transparency::Settings::new().with_base(constants::MAP_TRANSPARENCY);
    let map_settings = map::settings::Settings::new().with_transparency(map_transparency_settings);
    let sun_year = map::sun::IntensityYearPlanet::new(
        constants::MAP_SUN_TILT,
        constants::MAP_SUN_LATITUDE,
        constants::MAP_SUN_YEAR,
        constants::MAP_SUN_INTENSITY,
    );
    let sun_day = map::sun::IntensityDayPlanet::new(constants::MAP_SUN_DAY);
    let sun = map::sun::IntensityYearDay::new(sun_year, sun_day);
    let map = map::Map::new(constants::MAP_SIZE, map_settings, sun);

    // Setup the main loop
    let mut main_loop = application::MainLoop::new(
        map,
        camera,
        settings_window,
        settings_shader,
        settings_viewer,
    );

    // Run the application
    application::run(&mut main_loop);
}
