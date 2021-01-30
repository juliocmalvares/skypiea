extern crate image;
use image::{GenericImageView, RgbImage};


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


// #[test]
// #[warn(unused_must_use)]
// fn it_works () {
//     let img = image::open("files/input/galaxy/andro.jpg").unwrap();
//     let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);
//     buffer = img.to_rgb();
//     let grl = Brightness::new(buffer, 100);
//     buffer = grl.apply();

//     match buffer.save("files/output/andromeda-brightness-test.png") {
//         Ok(_) => (),
//         Err(_) => panic!("Test Brightness failed")
//     }
// }