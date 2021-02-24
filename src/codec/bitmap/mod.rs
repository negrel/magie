use std::convert::TryFrom;
use std::fmt::Error;
use std::result::Result;

use quick_error::quick_error;

use crate::color_space::Color;
use crate::image::Image;

pub mod decoder;

pub struct ImageBitmap {}

impl Image for ImageBitmap {
    fn size(self) -> (isize, isize) {
        unimplemented!()
    }

    fn width(self) -> isize {
        unimplemented!()
    }

    fn height(self) -> isize {
        unimplemented!()
    }

    fn at(self) -> Box<dyn Color> {
        unimplemented!()
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum BitmapTypeError {
        Invalid(a: (u8, u8)) {
            display("'{}{}' is not a valid BitmapType", a.0 as char, a.1 as char)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum BitmapType {
    BM = (('B' as u16) << 8) + 'M' as u16,
    BA = (('B' as u16) << 8) + 'A' as u16,
    CI = (('C' as u16) << 8) + 'I' as u16,
    CP = (('C' as u16) << 8) + 'P' as u16,
    IC = (('I' as u16) << 8) + 'C' as u16,
    PT = (('P' as u16) << 8) + 'T' as u16,
}

impl TryFrom<(u8, u8)> for BitmapType {
    type Error = BitmapTypeError;

    fn try_from(value: (u8, u8)) -> Result<Self, Self::Error> {
        let _type = ((value.0 as u16) << 8) + value.1 as u16;

        const BM: u16 = (('B' as u16) << 8) + 'M' as u16;
        const BA: u16 = (('B' as u16) << 8) + 'A' as u16;
        const CI: u16 = (('C' as u16) << 8) + 'I' as u16;
        const CP: u16 = (('C' as u16) << 8) + 'P' as u16;
        const IC: u16 = (('I' as u16) << 8) + 'C' as u16;
        const PT: u16 = (('P' as u16) << 8) + 'T' as u16;

        match _type {
            BM => Ok(Self::BM),
            BA => Ok(Self::BA),
            CI => Ok(Self::CI),
            CP => Ok(Self::CP),
            IC => Ok(Self::IC),
            PT => Ok(Self::PT),
            _ => Err(BitmapTypeError::Invalid(value)),
        }
    }
}

#[derive(Debug)]
pub struct BitmapHeader {
    _type: BitmapType,
    size: u32,
    pixel_offset: u32,
}

const HEADER_SIZE: usize = 14;

impl BitmapHeader {
    fn new(header: [u8; HEADER_SIZE]) -> Result<BitmapHeader, BitmapTypeError> {
        let _type = BitmapType::try_from((header[0], header[1]))?;
        let size = BitmapHeader::extract_size(&header);
        let pixel_offset = BitmapHeader::extract_pixel_offset(&header);

        Ok(BitmapHeader {
            _type: BitmapType::BM,
            size,
            pixel_offset,
        })
    }

    fn extract_size(header: &[u8]) -> u32 {
        const OFFSET: usize = 0x0002;

        let mut size = 0;
        for i in 0..4 {
            size += (header[OFFSET + i] as u32) << 8 * i;
        }

        size
    }

    fn extract_pixel_offset(header: &[u8]) -> u32 {
        const OFFSET: usize = 0x000A;

        let mut data_offset = 0;
        for i in 0..4 {
            data_offset += (header[OFFSET + i] as u32) << 8 * i;
        }

        data_offset
    }

    #[inline]
    pub fn get_type(&self) -> BitmapType {
        self._type
    }

    #[inline]
    pub fn get_size(&self) -> u32 {
        self.size
    }

    #[inline]
    pub fn get_pixel_offset(&self) -> u32 {
        self.pixel_offset
    }
}

mod bitmap_header_tests {
    use super::*;

    #[test]
    fn test_bitmap_header() {
        const HEADER: [u8; HEADER_SIZE] = [
            // Signature bytes
            ('B' as u8),
            ('M' as u8),
            // File size
            0xF0,
            0xE0,
            0xD0,
            0xE0,
            // Reserved bytes
            0,
            0,
            0,
            0,
            // Pixel array offset
            0xF0,
            0xA0,
            0xB0,
            0xC0,
        ];

        let header = BitmapHeader::new(HEADER).unwrap();
        assert_eq!(header.get_type(), BitmapType::BM);
        assert_eq!(header.get_size(), 0xE0_D0_E0_F0);
        assert_eq!(header.get_pixel_offset(), 0xC0_B0_A0_F0);
    }

    #[test]
    #[should_panic]
    fn test_bitmap_header_invalid_type() {
        const HEADER: [u8; HEADER_SIZE] = [
            // Signature bytes
            ('Z' as u8),
            ('Z' as u8),
            // File size
            0xF,
            0xE,
            0xD,
            0xE,
            // Reserved bytes
            0,
            0,
            0,
            0,
            // Pixel array offset
            0xF,
            0xA,
            0xB,
            0xC,
        ];

        let _header = BitmapHeader::new(HEADER).unwrap();
    }
}
