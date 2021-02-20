use num::traits::Bounded;

use crate::color_space::luma::Luma;
use crate::color_space::rgb::RGB;

#[derive(Clone, Debug)]
pub struct RGBA<T: Copy> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T,
}

impl<T: Copy> RGBA<T> {
    #[inline]
    fn new(r: T, g: T, b: T, a: T) -> Self {
        RGBA { r, g, b, a }
    }
}

impl<T: Bounded + Copy> From<RGB<T>> for RGBA<T> {
    fn from(src: RGB<T>) -> Self {
        return RGBA::new(src.r, src.g, src.b, T::max_value());
    }
}

impl<T: Bounded + Copy> From<Luma<T>> for RGBA<T> {
    fn from(src: Luma<T>) -> Self {
        RGBA::new(src.val, src.val, src.val, T::max_value())
    }
}
