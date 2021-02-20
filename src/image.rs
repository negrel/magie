use crate::color_space::Color;

pub trait Image {
    fn size(self) -> (isize, isize);

    fn width(self) -> isize;
    fn height(self) -> isize;

    fn at(self) -> Box<dyn Color>;
}
