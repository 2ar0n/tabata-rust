use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
    sdl2::{Keycode, Mod},
};

use std::thread::sleep;
use std::time::{Duration, SystemTime};

use tabata_core::{
    TabataApp, TabataInput,
    button::Button,
    embedded_graphics::{pixelcolor::BinaryColor, prelude::*},
    update_display,
};

const WIDTH: u32 = 120;
const HEIGHT: u32 = 160;
const TIMER_STEP_MS: u64 = 100;

fn main() -> Result<(), core::convert::Infallible> {
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Tabata Timer", &output_settings);

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(WIDTH, HEIGHT));
    let mut button = Button::new(1000);
    let mut tabata_app: TabataApp = Default::default();

    let mut is_spacebar_presed = false;
    'main_loop: loop {
        let next_wake_time = SystemTime::now() + Duration::from_millis(TIMER_STEP_MS);

        window.update(&display);

        let mut input: TabataInput = Default::default();

        for event in window.events() {
            match event {
                SimulatorEvent::KeyUp {
                    keycode: Keycode::Q,
                    keymod: Mod::NOMOD,
                    repeat: false,
                } => {
                    break 'main_loop;
                }
                SimulatorEvent::Quit => {
                    break 'main_loop;
                }
                SimulatorEvent::KeyDown {
                    keycode: Keycode::Space,
                    keymod: Mod::NOMOD,
                    repeat: false,
                } => {
                    is_spacebar_presed = true;
                }
                SimulatorEvent::KeyUp {
                    keycode: Keycode::Space,
                    keymod: Mod::NOMOD,
                    repeat: false,
                } => {
                    is_spacebar_presed = false;
                }
                SimulatorEvent::MouseWheel {
                    scroll_delta,
                    direction: _,
                } => {
                    input.steps = scroll_delta.y;
                }
                _ => {}
            }
        }

        input.button_press = button.update(TIMER_STEP_MS, is_spacebar_presed);
        tabata_app.update(TIMER_STEP_MS, &input);

        let _ = update_display(&mut display, &tabata_app);

        if let Ok(time_left) = next_wake_time.duration_since(SystemTime::now()) {
            sleep(time_left);
        }
    }

    Ok(())
}
