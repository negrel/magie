use std::io::Read;

use crate::codec::bitmap::*;

pub struct Decoder {}

pub fn decode(img: &mut dyn Read) -> Result<ImageBitmap, Error> {
    let mut buf_header: [u8; HEADER_SIZE] = [0; HEADER_SIZE];
    let _ = img.read_exact(&mut buf_header);

    let header = BitmapHeader::new(buf_header);

    match header {
        Ok(h) => println!("{:?}", h),
        Err(e) => panic!(e),
    }

    Ok(ImageBitmap {})
}
