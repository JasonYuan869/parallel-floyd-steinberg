use image::{Rgb, RgbImage};

/// An array of tuples containing the offset and the factor for the Floyd-Steinberg dithering algorithm.
pub const DITHERING_MATRIX: [([i32; 2], f32); 4] = [
    ([1, 0], 0.4375),
    ([-1, 1], 0.1875),
    ([0, 1], 0.3125),
    ([1, 1], 0.0625),
];

/// A trait for converting an image to the target color palette.
pub trait Converter {
    /// Returns a converted image in the target color palette.
    fn convert(&self, image: RgbImage) -> RgbImage;
}

/// A helper function that adds the error to the target pixel in all three channels with the given factor.
pub fn distribute_rgb_channels(pixel: &mut Rgb<u8>, errors: [f32; 3], factor: f32) {
    for (channel, error) in pixel.0.iter_mut().zip(errors) {
        let value = *channel as f32 / 256.0 + error * factor;
        if value >= 1.0 {
            *channel = 255;
        } else if value <= 0.0 {
            *channel = 0;
        } else {
            *channel = (value * 256.0) as u8;
        }
    }
}


