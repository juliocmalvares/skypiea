extern crate image;
// use image::{GenericImageView, RgbImage};
use crate::filters::base_filter::*;
use crate::io::fastbitmap::*;

impl<'a> Grayscale<'a> for BaseFilter {

    fn apply(mut fb: &FastBitmap<'a>) -> FastBitmap<'a> {
        // let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);
        
        for (i, j, pixel) in fb.img.enumerate_pixels() {
            let pix = fb.img.get_pixel(i, j);
            let average: u16 = (pix[0] as u16 + pix[1] as u16 + pix[2] as u16) / 3;
            // *pixel = image::Rgb([average as u8, average as u8, average as u8]);
            fb.set_pixel(i, j, image::Rgb([average as u8, average as u8, average as u8]));
        }
        fb.clone()
    }

}



// #[derive(Debug)]
// pub struct GrayScale {
//     img: RgbImage
// }

// impl GrayScale {
//     pub fn new(img: RgbImage) -> Self {
//         Self { img: img }
//     }

//     pub fn apply(&self) -> RgbImage {
//         let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
//         for (i, j, pixel) in buffer.enumerate_pixels_mut() {
//             let pix = self.img.get_pixel(i, j);
//             let average: u16 = (pix[0] as u16 + pix[1] as u16 + pix[2] as u16) / 3;
//             *pixel = image::Rgb([average as u8, average as u8, average as u8]);
//         }
//         buffer
//     }
// }


// #[test]
// #[warn(unused_must_use)]
// fn it_works () {
//     let mut fb = FastBitmap::new("files/input/galaxy/andro.jpg");
//     fb = Grayscale::apply(&fb);
// }