use winit::event_loop::EventLoop;

use crate::map;

mod window;
use window::{OptionalRenderedWindow, RenderedWindow};

mod settings;
use settings::{ShaderSettings, ViewerSettings, WindowSettings};
pub use settings::{ShaderSettingsInput, ViewerSettingsInput, WindowSettingsInput};

mod state;
use state::State;

mod main_loop;
pub use main_loop::MainLoop;

/// Runs the application
pub fn run<S: map::sun::Intensity>(main_loop: &mut MainLoop<S>) {
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
