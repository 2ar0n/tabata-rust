#![no_std]

use core::cmp::min;

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
    config_state: u8,
    state: State,
    cycle: u8,
    set: u8,
    remaining_time_ms: u64,
    is_paused: bool,
}

impl TabataApp {
    pub fn update(&mut self, elapsed_time_ms: u64, input: &TabataInput) {
        if self.state == State::Configuring {
            self.configure(&input);
        } else {
            match input.button_press {
                ButtonPressType::Press => {
                    self.is_paused = !self.is_paused;
                }
                ButtonPressType::LongPress => {
                    self.state = State::Configuring;
                }
                ButtonPressType::NotPressed => {
                    if !self.is_paused {
                        self.run_timer(elapsed_time_ms);
                    }
                }
            }
        }
    }

    fn configure(&mut self, input: &TabataInput) {
        if input.button_press == ButtonPressType::Press {
            self.config_state += 1;
            self.config_state = min(self.config_state, NUM_CONFIGURATION_STATES + 1);
        } else if input.button_press == ButtonPressType::LongPress {
            if self.config_state > 0 {
                self.config_state -= 1;
            }
        }

        if self.config_state > NUM_CONFIGURATION_STATES {
            self.state = State::Running;
            self.is_paused = false;
            self.remaining_time_ms = (self.config.work_time as u64) * 1000;
        }

        match self.config_state {
            0 => {
                update_timer(&mut self.config.work_time, input.steps);
            }
            1 => {
                update_timer(&mut self.config.rest_time, input.steps);
            }
            2 => {
                update_timer(&mut self.config.nb_cycles, input.steps / 2);
            }
            3 => {
                update_timer(&mut self.config.nb_sets, input.steps / 2);
            }
            4 => {
                update_timer(&mut self.config.rest_between_sets, input.steps);
            }
            _ => {}
        }
    }

    fn run_timer(&mut self, elapsed_time_ms: u64) {
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

const NUM_CONFIGURATION_STATES: u8 = 5;

fn update_timer(timer: &mut u8, steps: i32) {
    let temp = *timer as i32 + steps;
    if temp > 0 {
        if temp > 255 {
            *timer = 255u8;
        } else {
            *timer = temp as u8;
        }
    } else {
        *timer = 0u8;
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

    let size = display.bounding_box().size;
    let center = Point::new(size.width as i32 / 2, size.height as i32 / 2);

    display.clear(D::Color::from(BinaryColor::Off))?;

    if app.state == State::Configuring {
        match app.config_state {
            0 => {
                let text = "WORK TIME";
                let _ = Text::new(&text, Point::new(center.x, center.y), text_style).draw(display);
                let time = app.config.work_time;
                let mut buffer = [0u8; 5];
                let text = get_time_text(&mut buffer, time as u64);
                Text::new(&text, Point::new(center.x - 18, center.y - 10), text_style)
                    .draw(display)?;
            }
            1 => {
                let text = "REST TIME";
                let _ = Text::new(&text, Point::new(center.x, center.y), text_style).draw(display);
                let time = app.config.rest_time;
                let mut buffer = [0u8; 5];
                let text = get_time_text(&mut buffer, time as u64);
                Text::new(&text, Point::new(center.x - 18, center.y - 10), text_style)
                    .draw(display)?;
            }
            2 => {
                let text = "NB CYCLES";
                let _ = Text::new(&text, Point::new(center.x, center.y), text_style).draw(display);
                let time = app.config.nb_cycles;
                let mut buffer = [0u8; 5];
                let text = get_time_text(&mut buffer, time as u64);
                Text::new(&text, Point::new(center.x - 18, center.y - 10), text_style)
                    .draw(display)?;
            }
            3 => {
                let text = "NB SETS";
                let _ = Text::new(&text, Point::new(center.x, center.y), text_style).draw(display);
                let time = app.config.nb_sets;
                let mut buffer = [0u8; 5];
                let text = get_time_text(&mut buffer, time as u64);
                Text::new(&text, Point::new(center.x - 18, center.y - 10), text_style)
                    .draw(display)?;
            }
            4 => {
                let text = "REST in SETS";
                let _ = Text::new(&text, Point::new(center.x, center.y), text_style).draw(display);
                let time = app.config.rest_between_sets;
                let mut buffer = [0u8; 5];
                let text = get_time_text(&mut buffer, time as u64);
                Text::new(&text, Point::new(center.x - 18, center.y - 10), text_style)
                    .draw(display)?;
            }
            5 => {
                let text = "START?";
                let _ = Text::new(&text, Point::new(center.x, center.y), text_style).draw(display);
            }
            _ => {}
        }
    } else {
        let mut buffer = [0u8; 5];
        let time_text = get_time_text(&mut buffer, app.remaining_time_ms / 1000);
        Text::new(
            &time_text,
            Point::new(center.x - 18, center.y - 10),
            text_style,
        )
        .draw(display)?;

        // Draw the growing circle
        let max_radius = 100_f32;
        let max_time = (app.config.work_time as f32) * 1000_f32;
        let progress = (max_time - app.remaining_time_ms as f32) / max_time;
        let radius = (progress * max_radius) as i32;
        let circle_style = PrimitiveStyleBuilder::new()
            .stroke_color(D::Color::from(BinaryColor::On))
            .stroke_width(1)
            .build();
        let circle_center = center - Point::new(radius / 2, radius / 2);
        Circle::new(circle_center, radius as u32)
            .into_styled(circle_style)
            .draw(display)?;
    }

    Ok(())
}

fn get_time_text(buffer: &mut [u8; 5], remaining_time_ms: u64) -> &str {
    let minutes = (remaining_time_ms / 60) as u8;
    let seconds = ((remaining_time_ms) % 60) as u8;

    buffer[0] = b'0' + (minutes / 10);
    buffer[1] = b'0' + (minutes % 10);
    buffer[2] = b':';
    buffer[3] = b'0' + (seconds / 10);
    buffer[4] = b'0' + (seconds % 10);

    unsafe { core::str::from_utf8_unchecked(&buffer[..]) }
}
