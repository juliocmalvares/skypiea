pub mod filters;
extern crate image;

use image::{GenericImageView, RgbImage};


fn main() {
    let img = image::open("files/input/galaxy/andro.jpg").unwrap();
    let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);

    buffer = img.to_rgb();

    let grl = filters::grayscale_luminosity::GrayScaleLuminosity::new(buffer);
    buffer = grl.apply();
    buffer.save("files/output/andromeda-gravl.png");
    // let gr = filters::grayscale::GrayScale::new(buffer);
    // buffer = gr.apply();
    // buffer.save("files/output/andromeda-grav.png");


    // buffer = img.to_rgb();
    // let bhr = filters::brightness::Brightness::new(buffer, 100);
    // buffer = bhr.apply();
    // buffer.save("files/output/andro-bhr.png");

    // buffer = img.to_rgb();
    // let thr = filters::threshold::Threshold::new(buffer, 100);
    // buffer = thr.apply();
    // buffer.save("files/output/andro-thr.png");
}

