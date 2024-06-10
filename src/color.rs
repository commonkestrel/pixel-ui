pub use inner::*;

#[cfg(all(feature = "single-color", not(any(feature = "grayscale", feature = "full-color"))))]
mod inner {
    use std::ops::Not;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[repr(transparent)]
    pub struct Color(bool);

    impl Color {
        pub const WHITE: Color = Color(true);
        pub const BLACK: Color = Color(false);

        pub const fn new(value: bool) -> Color {
            Color(value)
        }
    }

    impl Into<u32> for Color {
        fn into(self) -> u32 {
            if self.0 {
                // max of every color
                0x00FFFFFF
            } else {
                0x00000000
            }
        }
    }

    impl Not for Color {
        type Output = Color;

        fn not(self) -> Self::Output {
            Color(!self.0)
        }
    }
}

#[cfg(all(feature = "grayscale"))]
mod inner {
    use std::ops::Not;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct Color(u8);
    
    impl Color {
        pub const WHITE: Color = Color(std::u8::MAX);
        pub const BLACK: Color = Color(std::u8::MIN);

        pub const fn new(value: u8) -> Color {
            Color(value)
        }
    }

    impl From<u8> for Color {
        fn from(value: u8) -> Self {
            Color(value)
        }
    }

    impl Into<u32> for Color {
        fn into(self) -> u32 {
            let color = self.0 as u32;
            0x00000000 | (color << 16) | (color << 8) | (color << 0)
        }
    }

    impl Not for Color {
        type Output = Color;

        fn not(self) -> Self::Output {
            Color(!self.0)
        }
    }
}
