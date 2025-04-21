
#[derive(Default, PartialEq)]
pub enum ButtonPressType {
    #[default]
    NotPressed,
    Press,
    LongPress,
}

pub struct Button {
    pub is_pressed: bool,
    pub time_since_pressed_ms: u64,
    time_for_long_press_ms: u64,
}

impl Button {
    pub fn new(time_for_long_press_ms: u64) -> Self {
        Button {
            is_pressed: false,
            time_since_pressed_ms: 0,
            time_for_long_press_ms: time_for_long_press_ms,
        }
    }

    pub fn update(&mut self, time_elapsed: u64, is_pressed: bool) -> ButtonPressType {
        let is_released: bool = self.is_pressed && !is_pressed;
        let is_long_press: bool = is_released && (self.time_since_pressed_ms >= self.time_for_long_press_ms);

        self.is_pressed = is_pressed;
        if self.is_pressed {
            self.time_since_pressed_ms += time_elapsed;
        } else {
            self.time_since_pressed_ms = 0;
        }

        if is_long_press {
            ButtonPressType::LongPress
        }
        else if is_released {
            ButtonPressType::Press
        } else {
            ButtonPressType::NotPressed
        }
    }
}
