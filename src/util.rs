use std::ops;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UVec2 {
    pub x: usize,
    pub y: usize,
}

impl UVec2 {
    #[inline]
    pub fn new(x: usize, y: usize) -> UVec2 {
        UVec2 { x, y }
    }

    #[inline]
    pub fn area(&self) -> usize {
        self.x * self.y
    }
}

impl From<(usize, usize)> for UVec2 {
    #[inline]
    fn from(value: (usize, usize)) -> Self {
        UVec2::new(value.0, value.1)
    }
}

impl Default for UVec2 {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl ops::Add for UVec2 {
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
pub struct IVec2 {
    pub x: isize,
    pub y: isize,
}

impl IVec2 {
    #[inline]
    pub fn new(x: isize, y: isize) -> IVec2 {
        IVec2 { x, y }
    }

    #[inline]
    pub fn area(&self) -> usize {
        (self.x * self.y).unsigned_abs()
    }
}

impl From<(isize, isize)> for IVec2 {
    #[inline]
    fn from(value: (isize, isize)) -> Self {
        IVec2::new(value.0, value.1)
    }
}

impl Default for IVec2 {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl ops::Add for IVec2 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Add<UVec2> for IVec2 {
    type Output = Self;

    fn add(self, rhs: UVec2) -> Self::Output {
        Self {
            x: self.x + rhs.x as isize,
            y: self.y + rhs.y as isize,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoundingBox {
    min: IVec2,
    max: IVec2,
}

impl BoundingBox {
    pub fn new(x1: isize, y1: isize, x2: isize, y2: isize) -> BoundingBox {
        let min = IVec2::new(x1.min(x2), y1.min(y2));
        let max = IVec2::new(x1.max(x2), y1.max(y2));

        BoundingBox { min, max }
    }

    pub fn from_size(origin: IVec2, size: UVec2) -> Self {
        Self {
            min: origin,
            max: origin + size,
        }
    }

    pub fn get_min(&self) -> IVec2 {
        self.min
    }

    pub fn get_max(&self) -> IVec2 {
        self.max
    }

    pub fn intersects(&self, target: IVec2) -> bool {
        let greater = self.min.x >= target.x && self.min.y >= target.y;
        let less = self.max.x <= target.x && self.max.y <= target.y;

        return greater && less;
    }

    pub fn size(&self) -> UVec2 {
        let width = self.max.x.abs_diff(self.min.x);
        let height = self.max.y.abs_diff(self.min.y);

        UVec2::new(width, height)
    }

    pub fn area(&self) -> usize {
        self.size().area()
    }
}

pub fn u8_to_bool_vec(content: &[u8]) -> Vec<bool> {
    let mut buf = Vec::with_capacity(content.len() * 8);
    for (i, byte) in content.iter().enumerate() {
        for bit in 0..8 {
            let value = (byte >> bit) & 0x01;
            // SAFETY: It is assured that `value` is either 0 or 1
            buf.push(unsafe { std::mem::transmute(value) });
        }
    }

    buf
}
