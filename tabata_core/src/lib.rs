use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, PrimitiveStyleBuilder},
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
};

pub use embedded_graphics;

pub enum Direction {
    Forward,
    Backward,
}

pub struct TabataState {
    previous_time: u64,
    button_is_pressed: bool,
    remaining_time: u64,
}

pub const WIDTH: u32 = 240;
pub const HEIGHT: u32 = 320;

pub fn update_display<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget,
    D::Color: PixelColor + From<BinaryColor>,
{
    let text_style = MonoTextStyle::new(&FONT_6X9, D::Color::from(BinaryColor::On));
    let center = Point::new(120, 160);
    let remaining_time = 10;
    // let time_text = format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60);
    let radius = 10;

    display.clear(D::Color::from(BinaryColor::Off))?;

    // Draw the growing circle
    let circle_style = PrimitiveStyleBuilder::new()
        .stroke_color(D::Color::from(BinaryColor::On))
        .stroke_width(1)
        .build();
    Circle::new(center, radius as u32)
        .into_styled(circle_style)
        .draw(display)?;

    // Draw the timer text
    let time_text = format!("{:02}:{:02}", remaining_time / 60, remaining_time % 60);
    Text::new(&time_text, Point::new(center.x - 18, center.y - 10), text_style)
        .draw(display)?;

    // Draw the exercise and rest indicators
    Text::new("Exercise", Point::new(center.x - 28, center.y + 20), text_style)
        .draw(display)?;

    Text::new("Rest", Point::new(center.x - 14, center.y + 40), text_style)
        .draw(display)?;

    Ok(())
}
