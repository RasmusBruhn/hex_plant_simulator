use std::time::Instant;

/// All values related to the running state of the application
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct State {
    /// All flags for the application
    pub flags: Flags,
    /// The next time the frame has increased
    pub next_frame_time: Instant,
    /// The next time the simulation must step
    pub next_sim_time: Instant,
}

impl State {
    /// Constructs a new viewer state with default values
    pub fn new() -> Self {
        return Self {
            flags: Flags::new(),
            next_frame_time: Instant::now(),
            next_sim_time: Instant::now(),
        };
    }
}

/// All flags for the application state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flags {
    /// If true, then the map has changed and the tile data must be updated on the GPU before next draw
    pub map_changed: bool,
    /// If true, then the simulation must be iterated once
    pub iterate_simulation: bool,
    /// If true then the simulation is constantly running
    pub run_simulation: bool,
    /// If true then the simulation has been updated and must be redrawn next frame
    pub redraw_simulation: bool,
    /// True if left shift is pressed down
    pub left_shift_active: bool,
}

impl Flags {
    /// Constructs a new set of flags with default values
    pub const fn new() -> Self {
        return Self {
            map_changed: false,
            iterate_simulation: false,
            run_simulation: false,
            redraw_simulation: false,
            left_shift_active: false,
        };
    }
}
