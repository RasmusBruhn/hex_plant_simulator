use crate::{map, types};

pub const FRAMERATE: f64 = 60.0;
pub const CAMERA_MOVE_SPEED: f64 = 1.0;
pub const CAMERA_ZOOM_SPEED: f64 = 1.0;
pub const CAMERA_BOOST_FACTOR: f64 = 2.0;
pub const CAMERA_ZOOM_LIMITS: (f64, f64) = (0.01, 1.0);
pub const COLOR_BACKGROUND: types::Color = types::Color::new(0.0, 0.0, 0.0, 1.0);
pub const COLOR_MAP_LIGHT: types::ColorMapLinearRGBA = types::ColorMapLinearRGBA {
    empty: types::Color::new(0.0, 0.0, 0.0, 1.0),
    saturated: types::Color::new(1.0, 1.0, 0.0, 1.0),
};
pub const COLOR_MAP_TRANSPARENCY: types::ColorMapLinearRGBA = types::ColorMapLinearRGBA {
    empty: types::Color::new(0.5, 0.5, 1.0, 1.0),
    saturated: types::Color::new(0.0, 0.0, 1.0, 1.0),
};
pub const COLOR_MODE_BACKGROUND: map::DataModeBackground = map::DataModeBackground::Light;

pub const MAP_SIZE: types::ISize = types::ISize { w: 200, h: 50 };
pub const MAP_TRANSPARENCY: f64 = 0.999;
pub const MAP_SUN_YEAR: f64 = 6000.0;
pub const MAP_SUN_DAY: f64 = 500.0;
pub const MAP_SUN_TILT: f64 = MATH_PI * 23.5 / 180.0;
pub const MAP_SUN_LATITUDE: f64 = MATH_PI * 55.7 / 180.0;
pub const MAP_SUN_INTENSITY: f64 = 1.0;

pub const SIM_RATE: f64 = 100.0;
pub const SIM_RATE_MODIFIER: f64 = 1.5;

pub const MATH_SQRT_3: f64 =
    1.73205080756887729352744634150587236694280525381038062805580697945193301690;
pub const MATH_PI: f64 =
    3.14159265358979323846264338327950288419716939937510582097494459230781640628;
