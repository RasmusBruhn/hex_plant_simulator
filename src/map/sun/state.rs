/// Describes the current state of the sun, it shines with an intensity
/// oscillating harmonically as a function of position
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    /// The base intensity of the sun
    pub intensity: f64,
    /// The position of the sun on the map in the range [0, 1[
    pub position: f64,
}

impl State {
    /// Constructs a new sun state
    pub fn new() -> Self {
        return Self {
            intensity: 1.0,
            position: 0.0,
        };
    }
}
