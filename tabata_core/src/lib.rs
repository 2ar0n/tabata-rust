#![no_std]

use button::ButtonPressType;
use embedded_graphics::{
    mono_font::{MonoTextStyle, ascii::FONT_6X9},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    text::Text,
};

pub use embedded_graphics;
pub mod button;

#[derive(Default)]
pub struct TabataApp {
    config: Config,
    state: State,
    cycle: u8,
    set: u8,
    remaining_time_ms: u64,
}

impl TabataApp {
    pub fn update(&mut self, elapsed_time_ms: u64, input: &TabataInput) {
        if self.state == State::Configuring {
            if (input.button_press == ButtonPressType::Press) {
                self.state = State::Running;
            }
        } else {
            if self.remaining_time_ms > elapsed_time_ms {
                self.remaining_time_ms = self.remaining_time_ms - elapsed_time_ms;
            } else {
                self.cycle += 1;
                if self.cycle >= self.config.nb_cycles {
                    self.set += 1;
                    self.cycle = 0;
                }
                if self.set >= self.config.nb_sets {
                    self.set = 0;
                    self.state = State::Configuring;
                    return;
                }
                // TODO: work/rest and set rest timers
                self.remaining_time_ms = (self.config.work_time as u64) * 1000;
            }
        }
    }
}

#[derive(Default, PartialEq)]
pub enum State {
    #[default]
    Configuring,
    Running,
}

#[derive(Default)]
pub struct TabataInput {
    pub button_press: ButtonPressType,
    pub steps: i32,
}

/// All times are in seconds
struct Config {
    pub work_time: u8,
    pub rest_time: u8,
    pub nb_cycles: u8,
    pub nb_sets: u8,
    pub rest_between_sets: u8,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            work_time: 30,
            rest_time: 7,
            nb_cycles: 4,
            nb_sets: 2,
            rest_between_sets: 60,
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
    // let max_radius = 100;

    // let progress = (app.timer.total_time_ms - app.timer.remaining_time_ms) as f32
    //     / app.timer.total_time_ms as f32;
    // let radius = (progress * max_radius as f32) as i32;

    let size = display.bounding_box().size;
    let center = Point::new(size.width as i32 / 2, size.height as i32 / 2);

    display.clear(D::Color::from(BinaryColor::Off))?;

    // // Draw the growing circle
    // let circle_style = PrimitiveStyleBuilder::new()
    //     .stroke_color(D::Color::from(BinaryColor::On))
    //     .stroke_width(1)
    //     .build();
    // let circle_center = center - Point::new(radius / 2, radius / 2);
    // Circle::new(circle_center, radius as u32)
    //     .into_styled(circle_style)
    //     .draw(display)?;

    let mut buffer = [0u8; 5];
    let time_text = get_time_text(&mut buffer, app.remaining_time_ms);
    Text::new(
        &time_text,
        Point::new(center.x - 18, center.y - 10),
        text_style,
    )
    .draw(display)?;

    Ok(())
}

fn get_time_text(buffer: &mut [u8; 5], remaining_time_ms: u64) -> &str {
    let minutes = (remaining_time_ms / 1000 / 60) as u8;
    let seconds = ((remaining_time_ms / 1000) % 60) as u8;

    buffer[0] = b'0' + (minutes / 10);
    buffer[1] = b'0' + (minutes % 10);
    buffer[2] = b':';
    buffer[3] = b'0' + (seconds / 10);
    buffer[4] = b'0' + (seconds % 10);

    unsafe { core::str::from_utf8_unchecked(&buffer[..]) }
}
