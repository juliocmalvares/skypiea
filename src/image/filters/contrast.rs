extern crate image;
use image::{GenericImageView,RgbImage};


#[derive(Debug)]
pub struct Contrast {
    img: RgbImage,
    scale: f32
}

impl Contrast {
    pub fn new(img: RgbImage, scale: f32) -> Contrast{
        Contrast { img: img, scale: scale }
    }

    pub fn apply(&self) -> RgbImage {
        let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
        for (i, j, pixel) in buffer.enumerate_pixels_mut() {
            let pix = self.img.get_pixel(i, j);
            // let average: u16 = (pix[0] as u16 + pix[1] as u16 + pix[2] as u16) / 3;
            *pixel = image::Rgb([(pix[0] as f32 * self.scale) as u8, (pix[1] as f32 * self.scale) as u8, (pix[2] as f32 * self.scale) as u8]);
        }
        
        buffer
    }
}


// #[test]
// #[warn(unused_must_use)]
// fn it_works () {
//     let img = image::open("files/input/galaxy/andromeda.jpg").unwrap();
//     let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);
//     buffer = img.to_rgb();
//     let grl = Contrast::new(buffer, 2.5);
//     buffer = grl.apply();
//     match buffer.save("files/output/andromeda-constrast-test.png") {
//         Ok(_) => (),
//         Err(_) => panic!("Test Contrast failed")
//     }
// }