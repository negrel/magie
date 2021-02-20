use std::fs::File;

use crate::image::Image;

mod jpeg;
pub mod bitmap;

// pub trait Encoder<T: Image> {
//     fn encode(_: ) -> T;
// }

pub trait Decoder<T: Image> {
    fn decode(_: File) -> T;
}
