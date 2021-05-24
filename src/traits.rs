pub(crate) trait Magnify {
    fn magnify(&self) {
        println!("magnifying...");
    }
}

impl Magnify for String {
    fn magnify(&self) {
        println!("m");
    }
}

impl Magnify for u8 {
    fn magnify(&self) {}
}

pub(crate) trait Slide<T, U> {
    fn slide(self, other: T) -> U;
}

impl Slide<usize, u8> for i8 {
    fn slide(self, other: usize) -> u8 {
        ((self as usize) + other) as u8
    }
}

pub struct Mountain {
    height: usize,
}
impl Mountain {
    pub fn new() -> Self {
        Mountain {
            height: 0
        }
    }
}

impl Magnify for Mountain {
    fn  magnify(&self)      {
        println!("magnified mountain");
    }
}