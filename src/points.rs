pub struct Point {
    pub x: u32,
    pub y: u32,
    pub value: String,
}

impl Point {
    pub fn new(x: u32, y: u32, value: String) -> Self {
        Self {
            x,
            y,
            value,
        }
    }
}
