#[derive(PartialEq, Copy, Clone, Debug)]
pub struct MapRect {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl MapRect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> MapRect {
        MapRect {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    // Returns true if this overlaps with other
    pub fn intersect(&self, other: &MapRect) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }

    // Return center of this rectangle
    pub fn center(&self) -> (i32, i32) {
        ((self.x1 + self.x2) / 2 + 1, (self.y1 + self.y2) / 2 + 1)
    }
}
