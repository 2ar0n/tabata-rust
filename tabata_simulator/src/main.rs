use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use std::thread::sleep;
use std::time::Duration;
use tabata_core::embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};

use tabata_core::update_display;

const WIDTH: u32 = 240;
const HEIGHT: u32 = 320;
const TIMER_DURATION: u64 = 20; // 20 seconds for demonstration

fn main() -> Result<(), core::convert::Infallible> {
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Tabata Timer", &output_settings);

    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(WIDTH, HEIGHT));

    for remaining_time in (0..=TIMER_DURATION).rev() {
        
        let _ = update_display(&mut display);

        window.update(&display);
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
