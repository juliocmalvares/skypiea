extern crate image;
use image::{RgbImage};

#[derive(Debug)]
pub struct Threshold {
    img: RgbImage,
    scale: u8
}

impl Threshold {
    pub fn new(img: RgbImage, scale: u8) -> Threshold{
        if scale < 255 {
            Threshold { img: img, scale: scale }
        } else {
            panic!("Scale must to be less then 255");
        }
    }

    pub fn apply(&self) -> RgbImage {
        let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
        for (i, j, pixel) in buffer.enumerate_pixels_mut() {
            let pix = self.img.get_pixel(i, j);
            *pixel = image::Rgb([if pix[0] > self.scale { u8::MAX } else { u8::MIN }, 
                                 if pix[1] > self.scale { u8::MAX } else { u8::MIN },
                                 if pix[2] > self.scale { u8::MAX } else { u8::MIN }]);
 
        }
        buffer
    }
}