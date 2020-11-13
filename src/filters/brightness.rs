extern crate image;
use image::{RgbImage};

#[derive(Debug)]
pub struct Brightness {
    img: RgbImage,
    scale: u8
}

impl Brightness {
    pub fn new(img: RgbImage, scale: u8) -> Brightness{
        if scale < 255 {
            Brightness { img: img, scale: scale }
        } else {
            panic!("Scale must to be less then 255");
        }
    }
    pub fn apply(&self) -> RgbImage {
        let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
        for (i, j, pixel) in buffer.enumerate_pixels_mut() {
            let pix = self.img.get_pixel(i, j);
            *pixel = image::Rgb([if (pix[0] as u16 + self.scale as u16) < 255 { (pix[0] + self.scale) as u8 } else { u8::MAX }, 
                                 if (pix[1] as u16 + self.scale as u16) < 255 { (pix[1] + self.scale) as u8 } else { u8::MAX },
                                 if (pix[2] as u16 + self.scale as u16) < 255 { (pix[2] + self.scale) as u8 } else { u8::MAX }]);
        }
        buffer
    
    }
}