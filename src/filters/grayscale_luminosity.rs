extern crate image;
use image::{GenericImageView,RgbImage};


#[derive(Debug)]
pub struct GrayScaleLuminosity {
    img: RgbImage
}

impl GrayScaleLuminosity {
    pub fn new(img: RgbImage) -> GrayScaleLuminosity{
        GrayScaleLuminosity { img: img }
    }

    pub fn apply(&self) -> RgbImage {
        let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
        for (i, j, pixel) in buffer.enumerate_pixels_mut() {
            let pix = self.img.get_pixel(i, j);
            // let average: u16 = (pix[0] as u16 + pix[1] as u16 + pix[2] as u16) / 3;
            *pixel = image::Rgb([(pix[0] as f32 * 0.21) as u8, (pix[1] as f32 * 0.72) as u8, (pix[2] as f32 * 0.07) as u8]);
        }
        buffer
    }
}


// #[test]
// #[warn(unused_must_use)]
// fn it_works () {
//     let img = image::open("files/input/galaxy/andro.jpg").unwrap();
//     let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);
//     buffer = img.to_rgb();
//     let grl = GrayScaleLuminosity::new(buffer);
//     buffer = grl.apply();
//     match buffer.save("files/output/andromeda-grl-test.png") {
//         Ok(_) => (),
//         Err(_) => panic!("Test Grayscale Luminosity failed")
//     }
// }