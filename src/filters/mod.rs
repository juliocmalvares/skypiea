pub mod base_filter;
pub use self::base_filter::*;

pub mod convolution;


pub mod grayscale;
pub use self::grayscale::*;

pub mod brightness;
pub use self::brightness::*;

pub mod threshold;
pub use self::threshold::*;

pub mod grayscale_luminosity;
pub use self::grayscale_luminosity::*;

pub mod contrast;
pub use self::contrast::*;


pub mod invert;
pub use self::invert::*;

pub mod histogram;
pub use self::histogram::*;
// use crate::io::*;