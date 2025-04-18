use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window, sdl2::{Keycode, Mod},
};

use std::thread::sleep;
use std::time::{Duration, SystemTime};

use tabata_core::embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use tabata_core::{TabataState, update_display};

const WIDTH: u32 = 120;
const HEIGHT: u32 = 160;
const TIMER_DURATION_MS: u64 = 5000;
const TIMER_STEP_MS: u64 = 100;

fn main() -> Result<(), core::convert::Infallible> {
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Tabata Timer", &output_settings);

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(WIDTH, HEIGHT));

    let mut state: TabataState = Default::default();
    state.start(TIMER_DURATION_MS);

    'timer_loop: while state.is_running {
        let target_time = SystemTime::now() + Duration::from_millis(TIMER_STEP_MS);

        let _ = update_display(&mut display, &state);
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::KeyUp {
                    keycode: Keycode::Q,
                    keymod: Mod::NOMOD,
                    repeat: false,
                } => {
                    break 'timer_loop;
                }
                SimulatorEvent::Quit => {
                    break 'timer_loop;
                }
                _ => {}
            }
        }

        let current_time = SystemTime::now();
        if let Ok(duration) = target_time.duration_since(current_time) {
            sleep(duration);
        }

        state.update(TIMER_STEP_MS);
    }

    Ok(())
}
