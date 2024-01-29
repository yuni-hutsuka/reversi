pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        let point: Point = Self { x, y };

        return point;
    }

    pub fn copy(&self) -> Point {
        return Point::new(self.x, self.y)
    }

    pub fn equal(&self, target: Point) -> bool {
        if self.x == target.x && self.y == target.y {
            return true;
        } else {
            return false;
        }
    }
}
