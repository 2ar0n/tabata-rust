
use tabata_core::embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};
use std::thread::sleep;
use std::time::Duration;

const WIDTH: u32 = 240;
const HEIGHT: u32 = 320;
const TIMER_DURATION: u64 = 20; // 20 seconds for demonstration

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(WIDTH, HEIGHT));

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
    let center = Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2);
    let max_radius = 100;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Tabata Timer", &output_settings);

    for remaining_time in (0..=TIMER_DURATION).rev() {
        let progress = (TIMER_DURATION - remaining_time) as f32 / TIMER_DURATION as f32;
        let radius = (progress * max_radius as f32) as i32;

        // Clear the display
        display.clear(BinaryColor::Off)?;

        // Draw the growing circle
        let circle_style = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::On)
            .stroke_width(1)
            .build();
        Circle::new(center, radius as u32)
            .into_styled(circle_style)
            .draw(&mut display)?;

        // Draw the timer text
        let time_text = format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60);
        Text::new(&time_text, Point::new(center.x - 18, center.y - 10), text_style)
            .draw(&mut display)?;

        // Draw the exercise and rest indicators
        Text::new("Exercise", Point::new(center.x - 28, center.y + 20), text_style)
            .draw(&mut display)?;

        Text::new("Rest", Point::new(center.x - 14, center.y + 40), text_style)
            .draw(&mut display)?;

        // Update the window
        window.update(&display);

        // Sleep for 1 second
        sleep(Duration::from_secs(1));
    }

    Ok(())
}
