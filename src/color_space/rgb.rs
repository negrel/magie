use num::traits::NumOps;

use crate::color_space::luma::Luma;
use crate::color_space::ycbcr::YCbCr;

#[derive(Clone, Debug)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> RGB<T> {
    #[inline]
    fn new(r: T, g: T, b: T) -> Self {
        RGB {
            r,
            g,
            b,
        }
    }
}

impl<T: Copy> From<Luma<T>> for RGB<T> {
    fn from(src: Luma<T>) -> Self {
        RGB::new(src.val, src.val, src.val)
    }
}

impl<T: Into<f32> + NumOps + Copy + From<f32>, S: Into<f32> + NumOps + Copy> From<YCbCr<T, S>> for RGB<T> {
    fn from(src: YCbCr<T, S>) -> Self {
        let y = Into::<f32>::into(src.y);
        let cr = Into::<f32>::into(src.cr);
        let cb = Into::<f32>::into(src.cb);

        let r = y + 1.402 * (cr - 128.0);
        let g = y - 0.34414 * (cb - 128.0) - 0.71414 * (cr - 128.0);
        let b = y + 1.1772 * (cb - 128.0);

        RGB::new(T::from(r), T::from(g), T::from(b))
    }
}
