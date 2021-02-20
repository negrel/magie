mod luma;
mod rgb;
mod rgba;
mod ycbcr;

pub trait Color {
    fn to_rgba8(&self) -> rgba::RGBA<u8>;
    fn to_rgba16(&self) -> rgba::RGBA<u16>;
}
