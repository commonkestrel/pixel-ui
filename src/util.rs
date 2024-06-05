use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    #[inline]
    pub fn new(x: usize, y: usize) -> Vec2 {
        Vec2 { x, y }
    }
}

impl From<(usize, usize)> for Vec2 {
    #[inline]
    fn from(value: (usize, usize)) -> Self {
        Vec2::new(value.0, value.1)
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    min: Vec2,
    max: Vec2,
}

impl BoundingBox {
    pub fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> BoundingBox {
        let min = Vec2::new(x1.min(x2), y1.min(y2));
        let max = Vec2::new(x1.max(x2), y1.max(y2));

        BoundingBox { min, max }
    }

    pub fn from_size(origin: Vec2, size: Vec2) -> Self {
        Self {
            min: origin,
            max: origin + size,
        }
    }

    pub fn get_min(&self) -> Vec2 {
        self.min
    }

    pub fn get_max(&self) -> Vec2 {
        self.max
    }

    pub fn intersects(&self, target: Vec2) -> bool {
        let greater = self.min.x >= target.x && self.min.y >= target.y;
        let less = self.max.x <= target.x && self.max.y <= target.y;

        return greater && less;
    }
}
