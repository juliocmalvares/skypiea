extern crate image;
use image::{RgbImage};

#[derive(Debug)]
pub struct GrayScale {
    img: RgbImage
}

impl GrayScale {
    pub fn new(img: RgbImage) -> GrayScale{
        GrayScale { img: img }
    }

    pub fn apply(&self) -> RgbImage {
        let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
        for (i, j, pixel) in buffer.enumerate_pixels_mut() {
            let pix = self.img.get_pixel(i, j);
            let average: u16 = (pix[0] as u16 + pix[1] as u16 + pix[2] as u16) / 3;
            *pixel = image::Rgb([average as u8, average as u8, average as u8]);
        }
        buffer
    }
}
