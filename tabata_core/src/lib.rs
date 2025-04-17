pub trait Update {
    fn update(&mut self, button_pressed: bool, direction: Direction, speed: u32);
}

pub enum Direction {
    Forward,
    Backward,
}
