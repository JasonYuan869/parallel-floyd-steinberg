use crate::colors::get_color_tree;
use crate::convert::{Converter, distribute_rgb_channels, DITHERING_MATRIX};
use image::RgbImage;

/// The standard single-threaded converter that implements the Floyd-Steinberg dithering algorithm.
pub struct SingleThreadedConverter;

impl SingleThreadedConverter {
    pub fn new() -> Self {
        SingleThreadedConverter
    }
}

impl Converter for SingleThreadedConverter {
    fn convert(&self, mut image: RgbImage) -> RgbImage {
        let (width, height) = image.dimensions();

        for x in 0..width {
            for y in 0..height {
                let color = image.get_pixel(x, y);

                // Difference is the vector difference between the target color
                // and the closest color in the palette expressed in RGB space
                let (closest_color, difference) = get_color_tree().find_closest(color);

                *image.get_pixel_mut(x, y) = closest_color;

                // Normalize the error to the range [0, 1]
                let errors = difference.map(|err| err as f32 / 256.0);

                // Propagate errors to each of the four pixels according to Floyd-Steinberg
                for ([vx, vy], factor) in DITHERING_MATRIX {
                    let x = x as i32 + vx;
                    let y = y as i32 + vy;

                    // Check bounds within image (y will never be negative)
                    if x < 0 || x as u32 >= width || y as u32 >= height {
                        continue;
                    }

                    let original_color = image.get_pixel_mut(x as u32, y as u32);
                    distribute_rgb_channels(original_color, errors, factor);
                }
            }
        }
        image
    }
}
