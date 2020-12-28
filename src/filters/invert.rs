extern crate image;
use image::{GenericImageView,RgbImage};


#[derive(Debug)]
pub struct Invert {
    img: RgbImage
}

impl Invert {
    pub fn new(img: RgbImage) -> Invert{
        Invert { img: img }
    }

    pub fn apply(&self) -> RgbImage {
        let mut buffer: RgbImage = image::ImageBuffer::new(self.img.dimensions().0, self.img.dimensions().1);
        for (i, j, pixel) in buffer.enumerate_pixels_mut() {
            let pix = self.img.get_pixel(i, j);
            // let average: u16 = (pix[0] as u16 + pix[1] as u16 + pix[2] as u16) / 3;
            *pixel = image::Rgb([(u8::MAX - pix[0]) as u8, (u8::MAX - pix[1]) as u8, (u8::MAX - pix[2]) as u8]);
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
//     let grl = Invert::new(buffer);
//     buffer = grl.apply();
//     match buffer.save("files/output/andromeda-invert-test.png") {
//         Ok(_) => (),
//         Err(_) => panic!("Test Contrast failed")
//     }
// }