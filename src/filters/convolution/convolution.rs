// use io;
use crate::io::fastbitmap::*;
extern crate image;
use image::{GenericImageView, RgbImage};

use crate::filters::grayscale;
#[derive(Debug)]
pub struct Convolution {
    img: RgbImage,
    kernel: Vec<Vec<i8>>,
    weight: i8
}

impl Convolution {
    pub fn new (img: RgbImage, kernel: Vec<Vec<i8>>) -> Convolution {
        let mut w = 0;
        for lin in 0..kernel.len() {
            for col in 0..kernel.len() {
                w += kernel[lin][col] as i8;
            }
        }
        Convolution {img: img, kernel: kernel, weight: w}
    }

    pub fn apply(&self) -> RgbImage {
        let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
        for (i, j, pixel) in buffer.enumerate_pixels_mut() {
            *pixel = image::Rgb([1,1,1]);
        }
        let mut centroit: i16 = 0;
        println!("{:?}", self.kernel);
        for i in 0..self.img.dimensions().0 + 1 - self.kernel.len() as u32 {
            for j in 0..self.img.dimensions().1 + 1 - self.kernel.len() as u32 {
                for lin in 0..self.kernel.len() {
                    for col in 0..self.kernel.len() {
                        centroit += (self.img.get_pixel(lin as u32 + i, col as u32 + j)[0]) as i16 * self.kernel[lin][col] as i16;
                    }
                }
                // println!("{:?} {:?}", self.weight, centroit as u8);
                if self.weight != 0 {
                    centroit = centroit / self.weight as i16;
                }
                buffer.put_pixel(i, j, image::Rgb([centroit as u8, centroit as u8, centroit as u8]));
                // let pixel = buffer.get_pixel_mut(i,j);
                // *pixel = image::Rgb([centroit as u8, centroit as u8, centroit as u8]);
                centroit = 0;
            }
        }
        buffer
    }
}


// #[test]
// #[warn(unused_must_use)]
// fn it_works () {
//     let img = image::open("files/input/chloe.jpg").unwrap();
//     let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);
//     buffer = img.to_rgb();
//     let gray = grayscale::GrayScale::new(buffer);
//     buffer = gray.apply();
//     let grl = Convolution::new(buffer, vec![[-1, -1, -1].to_vec(), [-1, 8, -1].to_vec(), [-1, -1, -1].to_vec()]);
//     buffer = grl.apply();

//     match buffer.save("files/output/chloe-convolution-gray-test.png") {
//         Ok(_) => (),
//         Err(_) => panic!("Test Brightness failed")
//     }
// }