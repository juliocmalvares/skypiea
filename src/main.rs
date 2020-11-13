pub mod filters;
extern crate image;

use image::{GenericImageView, RgbImage};


fn main() {
    let img = image::open("drow.png").unwrap();
    let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);

    buffer = img.to_rgb();

    let gr = filters::grayscale::GrayScale::new(buffer);
    buffer = gr.apply();
    buffer.save("grav.png");


    buffer = img.to_rgb();
    let bhr = filters::brightness::Brightness::new(buffer, 100);
    buffer = bhr.apply();
    buffer.save("bhr.png");
}
