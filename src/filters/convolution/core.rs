use image::ImageBuffer;
mod newton_kernel;

struct Convolutional {
    img: ImageBuffer,
    kernel: std::vec::Vec<std::vec::Vec<f64>>
}


impl newton_kernel::NewtonKernel for Convolutional {
    fn new() {
        println!("Deu pau não");
    }

    fn apply() {

    }
}




