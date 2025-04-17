// use embedded_graphics::{
//     pixelcolor::BinaryColor,
//     prelude::*,
//     primitives::{Circle, PrimitiveStyleBuilder},
//     mono_font::{ascii::FONT_6X9, MonoTextStyle},
//     text::Text,
// };

pub use embedded_graphics;

// pub enum Direction {
//     Forward,
//     Backward,
// }

// pub const WIDTH: u32 = 240;
// pub const HEIGHT: u32 = 320;

// pub fn update_display(display: &mut D) -> Result<Self::Output, D::Error>
// where
//     D: DrawTarget<Color = Self::Color>
// {
//     let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
//     let center = Point::new(WIDTH as i32 / 2, HEIGHT as i32 / 2);
//     let max_radius = 100;

//     // Clear the display
//     display.clear(BinaryColor::Off)?;

//     // Draw the growing circle
//     let circle_style = PrimitiveStyleBuilder::new()
//         .stroke_color(BinaryColor::On)
//         .stroke_width(1)
//         .build();
//     Circle::new(center, radius as u32)
//         .into_styled(circle_style)
//         .draw(&mut display)?;

//     // Draw the timer text
//     // let time_text = format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60);
//     // Text::new(&time_text, Point::new(center.x - 18, center.y - 10), text_style)
//     //     .draw(&mut display)?;

//     // Draw the exercise and rest indicators
//     Text::new("Exercise", Point::new(center.x - 28, center.y + 20), text_style)
//         .draw(&mut display)?;

//     Text::new("Rest", Point::new(center.x - 14, center.y + 40), text_style)
//         .draw(&mut display)?;

//     Ok(())
// }
