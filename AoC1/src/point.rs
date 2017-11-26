#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize
}
impl Point {
    pub const ZERO: Point = Point {x: 0, y: 0};
    pub fn get_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}
impl Default for Point {
    fn default() -> Point {
        Point::ZERO
    }
}
