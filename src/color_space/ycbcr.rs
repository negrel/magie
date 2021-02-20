use num::ToPrimitive;
use num::traits::NumOps;

use crate::color_space::rgb::RGB;

#[derive(Clone, Debug)]
pub struct YCbCr<T: NumOps, S: NumOps> {
    pub y: T,
    pub cb: S,
    pub cr: S,
}

impl<T: NumOps, S: NumOps> YCbCr<T, S> {
    #[inline]
    fn new(y: T, cb: S, cr: S) -> Self {
        YCbCr {
            y,
            cb,
            cr,
        }
    }
}


impl<T: From<f32> + ToPrimitive + NumOps + Copy, S: NumOps + From<f32>> From<RGB<T>> for YCbCr<T, S> {
    fn from(src: RGB<T>) -> Self {
        let r = src.r.to_f32().unwrap();
        let g = src.g.to_f32().unwrap();
        let b = src.b.to_f32().unwrap();

        let y = 0.299 * r + 0.587 * g + 0.114 * b;
        let cb = -0.1687 * r - 0.3313 * g + 0.5 * b + 128.0;
        let cr = 0.5 * r - 0.4187 * g - 0.0813 * b + 128.0;

        YCbCr::new(T::from(y), S::from(cb), S::from(cr))
    }
}
