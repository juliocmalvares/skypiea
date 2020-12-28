extern crate image;
use image::{GenericImageView, RgbImage};
use crate::filters::grayscale;

pub struct GrHistogram {
    img: RgbImage,
    result: Vec::<i32>

}


impl GrHistogram {
    pub fn new(img: RgbImage) -> GrHistogram {
        let mut v = Vec::new();
        for i in 0..254 {
            v.push(0);
        }

        GrHistogram{ img: img, result: v}
    }

    pub fn apply(&self) -> Vec<i32> {

        let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
        
        for (i,j,pixel) in self.img.enumerate_pixels() {
            buffer.put_pixel(i, j, *pixel);
        }
        // let gr = grayscale::GrayScale::new(buffer);
        // buffer = gr.apply();
        let mut v = Vec::new();
        for _i in 0..256 {
            v.push(0);
        }
        for (i,j,pixel) in buffer.enumerate_pixels() {
            v[pixel[0] as usize] += 1;
        }
        v
   }
}