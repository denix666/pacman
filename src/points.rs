pub struct Point {
    pub x: i32,
    pub y: i32,
    pub value: String,
}

impl Point {
    pub fn new(x: i32, y: i32, value: String) -> Self {
        Self {
            x,
            y,
            value,
        }
    }
}
