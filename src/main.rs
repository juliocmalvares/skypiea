pub mod filters;
extern crate image;

use image::{GenericImageView, RgbImage};
pub mod io;


fn main() {
    // let img = image::open("files/input/galaxy/andro.jpg").unwrap();
    // let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);

    // buffer = img.to_rgb();
    // let fbm = io::fastbitmap::FastBitmap::new(buffer);

    // let grl = filters::grayscale_luminosity::GrayScaleLuminosity::new(buffer);
    // buffer = grl.apply();
    // buffer.save("files/output/andromeda-gravl.png");
    // println!("{:?}", (200 as f32 * 2.5) as u8);
    // buffer.save("files/output/andromeda-grav.png");
    
    
    // buffer = img.to_rgb();
    // let bhr = filters::brightness::Brightness::new(buffer, 100);
    // buffer = bhr.apply();
    // buffer.save("files/output/andro-bhr.png");
    
    // buffer = img.to_rgb();
    // let thr = filters::threshold::Threshold::new(buffer, 100);
    // buffer = thr.apply();
    // buffer.save("files/output/andro-thr.png");
    // let img = image::open("files/input/galaxy/andro.jpg").unwrap();
    // let mut buffer: RgbImage = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);
    // buffer = img.to_rgb();

    // let hist = filters::histogram::GrHistogram::new(buffer);
    // let mut h = Vec::new();
    // h = hist.apply();
    // println!("{:?}", h);
    // let gr = filters::grayscale::GrayScale::new(buffer);
    // buffer = gr.apply();
    // let grl = filters::convolution::Convolution::new(buffer, vec![[-1, -1, -1].to_vec(), [-1, 8, -1].to_vec(), [-1, -1, -1].to_vec()]);
    // buffer = grl.apply();

    // match buffer.save("files/output/engr-convolution-test.png") {
    //     Ok(_) => (),
    //     Err(_) => panic!("Test Brightness failed")
    // }

}

