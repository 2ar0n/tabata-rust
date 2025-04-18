use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::{Keycode, Mod};

use std::thread::sleep;
use std::time::{Duration, SystemTime};

use tabata_core::embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use tabata_core::{TabataState, update_display};

const WIDTH: u32 = 240;
const HEIGHT: u32 = 320;
const TIMER_DURATION_MS: u64 = 5000;
const TIMER_STEP_MS: u64 = 100;

fn main() -> Result<(), core::convert::Infallible> {
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Tabata Timer", &output_settings);

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(WIDTH, HEIGHT));

    let mut state = TabataState {
        remaining_time_ms: 0,
        total_time_ms: TIMER_DURATION_MS,
    };
    for remaining_time in (0..=TIMER_DURATION_MS)
        .rev()
        .step_by(TIMER_STEP_MS as usize)
    {
        let target_time = SystemTime::now() + Duration::from_millis(TIMER_STEP_MS);

        state.remaining_time_ms = remaining_time;
        let _ = update_display(&mut display, &state);
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::KeyUp {
                    keycode: Keycode::Q,
                    keymod: Mod::NOMOD,
                    repeat: false,
                } => {
                    break;
                }
                SimulatorEvent::Quit => {
                    break;
                }
                _ => {}
            }
        }

        let current_time = SystemTime::now();
        if let Ok(duration) = target_time.duration_since(current_time) {
            sleep(duration);
        }
    }

    Ok(())
}
