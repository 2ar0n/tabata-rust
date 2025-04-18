// #![no_std]

use std::sync::Arc;

use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};

pub use embedded_graphics;

#[derive(Default)]
pub struct TabataApp {
    timer: TabataTimer,
}

impl TabataApp {
    pub fn update(&mut self, elapsed_time: u64, input: &TabataInput) {
        if !self.timer.is_running {
            if input.steps > 0 {
                self.timer.total_time_ms += input.steps as u64 * 1000
            } else if -input.steps * 1000 < self.timer.total_time_ms as i32 {
                self.timer.total_time_ms = self.timer.total_time_ms - (-input.steps * 1000) as u64
            } else {
                self.timer.total_time_ms = 1
            }

            self.timer.remaining_time_ms = self.timer.total_time_ms;

            if input.button_pressed {
                self.timer.start();
            }
        } else {
            self.timer.update(elapsed_time);
        }
    }
}

#[derive(Default)]
pub struct TabataInput {
    pub button_pressed: bool,
    pub steps: i32,
}

struct TabataTimer {
    pub remaining_time_ms: u64,
    pub total_time_ms: u64,
    pub is_running: bool,
}

impl Default for TabataTimer {
    fn default() -> Self {
        TabataTimer {
            remaining_time_ms: 5000,
            total_time_ms: 5000,
            is_running: false,
        }
    }
}

impl TabataTimer {
    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn update(&mut self, elapsed_time: u64) {
        if self.remaining_time_ms > elapsed_time {
            self.remaining_time_ms = self.remaining_time_ms - elapsed_time;
        } else {
            self.remaining_time_ms = 0;
            self.is_running = false;
        }
    }
}

pub fn update_display<D>(display: &mut D, app: &TabataApp) -> Result<(), D::Error>
where
    D: DrawTarget,
    D::Color: PixelColor + From<BinaryColor>,
{
    let text_style: MonoTextStyle<'_, <D as DrawTarget>::Color> =
        MonoTextStyle::new(&FONT_6X9, D::Color::from(BinaryColor::On));
    let max_radius = 100;

    let progress = (app.timer.total_time_ms - app.timer.remaining_time_ms) as f32
        / app.timer.total_time_ms as f32;
    let radius = (progress * max_radius as f32) as i32;

    let size = display.bounding_box().size;
    let center = Point::new(size.width as i32 / 2, size.height as i32 / 2);

    display.clear(D::Color::from(BinaryColor::Off))?;

    // Draw the growing circle
    let circle_style = PrimitiveStyleBuilder::new()
        .stroke_color(D::Color::from(BinaryColor::On))
        .stroke_width(1)
        .build();
    let circle_center = center - Point::new(radius / 2, radius / 2);
    Circle::new(circle_center, radius as u32)
        .into_styled(circle_style)
        .draw(display)?;

    // Draw the timer text
    let time_text = format!(
        "{:02}:{:02}",
        app.timer.remaining_time_ms / 1000 / 60,
        (app.timer.remaining_time_ms / 1000) % 60
    );
    Text::new(
        &time_text,
        Point::new(center.x - 18, center.y - 10),
        text_style,
    )
    .draw(display)?;

    // Draw the exercise and rest indicators
    Text::new(
        "Exercise",
        Point::new(center.x - 28, center.y + 20),
        text_style,
    )
    .draw(display)?;

    Text::new("Rest", Point::new(center.x - 14, center.y + 40), text_style).draw(display)?;

    Ok(())
}
