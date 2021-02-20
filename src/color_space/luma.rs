use num::traits::NumOps;

use crate::color_space::rgb::RGB;

#[derive(Debug)]
pub struct Luma<T> {
    pub val: T,
}

impl<T> Luma<T> {
    #[inline]
    fn new(luma: T) -> Self {
        Luma { val: luma }
    }
}

impl<T: From<isize> + NumOps + Copy> From<RGB<T>> for Luma<T> {
    fn from(src: RGB<T>) -> Self {
        let luma = (src.r + src.g + src.b) / T::from(3);
        Luma::new(luma)
    }
}
