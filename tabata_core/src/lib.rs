use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{circle, Circle, PrimitiveStyleBuilder},
    text::Text,
};

pub use embedded_graphics;

pub enum Direction {
    Forward,
    Backward,
}

pub struct TabataState {
    // previous_time: u64,
    // button_is_pressed: bool,
    pub remaining_time: u64,
    pub total_time: u64,
}

pub fn update_display<D>(display: &mut D, state: &TabataState) -> Result<(), D::Error>
where
    D: DrawTarget,
    D::Color: PixelColor + From<BinaryColor>,
{
    let text_style: MonoTextStyle<'_, <D as DrawTarget>::Color> = MonoTextStyle::new(&FONT_6X9, D::Color::from(BinaryColor::On));
    let max_radius = 100;

    let progress = (state.total_time - state.remaining_time) as f32 / state.total_time as f32;
    let radius = (progress * max_radius as f32) as i32;

    let size = display.bounding_box().size;
    let center = Point::new(size.width as i32 / 2, size.height as i32 / 2);

    display.clear(D::Color::from(BinaryColor::Off))?;

    // Draw the growing circle
    let circle_style = PrimitiveStyleBuilder::new()
        .stroke_color(D::Color::from(BinaryColor::On))
        .stroke_width(1)
        .build();
    let circle_center = center - Point::new(radius/2, radius/2);
    Circle::new(circle_center, radius as u32)
        .into_styled(circle_style)
        .draw(display)?;

    // Draw the timer text
    let time_text = format!("{:02}:{:02}", state.remaining_time / 60, state.remaining_time % 60);
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
