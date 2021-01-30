extern crate image;
use image::{GenericImageView, RgbImage};
use crate::io::fastbitmap::*;

#[derive(Debug)]
pub struct BaseFilter {
    scale: f32
}

pub trait Grayscale<'a> {
    // pub fn new() -> Self;
    fn apply(fb: &FastBitmap<'a>) -> FastBitmap<'a>;
}