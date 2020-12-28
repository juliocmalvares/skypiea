extern crate image;
use image::{RgbImage};


#[derive(Copy, Clone)]
pub struct FastBitmap<'a> {
    pub img: &'a RgbImage,
    pub buffer: &'a RgbImage
}

impl FastBitmap <'_>{

    pub fn new<'a>(path: &str) -> Self {
        let img = image::open(path).unwrap();
        let mut buffer: RgbImage = image::ImageBuffer::new(img.to_rgb().dimensions().0, img.to_rgb().dimensions().1);

        Self {img: &img.to_rgb(), buffer: &buffer}
    }

    pub fn save_img <'a> (&self, path: &str) {
        match self.buffer.save(path) {
            Ok(_) => (),
            Err(_) => panic!("Impossible to save image with path {}", path)
        }
    }
    
    pub fn get_pixel <'a> (&self, i: u32, j: u32) -> image::Rgb<u8> {
        *self.img.get_pixel(i, j)
    }

    pub fn set_pixel <'a> (mut self, i: u32, j: u32, p: image::Rgb<u8>) {
        self.buffer.put_pixel(i, j, p);
    }


    // pub fn apply_filter(self, f: fn(RgbImage)) {
    //     let mut filter = f(&self.img);

    // }
}



#[test]
#[warn(unused_must_use)]
fn it_works () {
    let mut fb = FastBitmap::new("files/input/galaxy/andromeda.jpg");
    println!("{:?}", fb.buffer);
    fb.save_img("files/output/test_saida_fastbitmap.png")
}