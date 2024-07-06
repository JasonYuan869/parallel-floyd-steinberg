use std::sync::{Arc, Mutex};
use crossbeam::channel::{Receiver, Sender, unbounded};
use image::{Rgb, RgbImage};
use parking_lot::RwLock;
use rayon::Scope;
use crate::colors::get_color_tree;
use crate::convert::{Converter, distribute_rgb_channels, DITHERING_MATRIX};

/// A converter that converts the image to the target color palette
/// using multiple threads and channels to communicate between them.
pub struct ChannelConverter;

impl ChannelConverter {
    pub fn new() -> Self {
        ChannelConverter {}
    }
}

impl Converter for ChannelConverter {
    fn convert(&self, image: RgbImage) -> RgbImage {
        let (width, height) = image.dimensions();
        let orginal_image = Arc::new(RwLock::new(image));


        rayon::scope(|s| {
            s.spawn(|s| {
                let cloned_image_ref = orginal_image.clone();
                thread(s, cloned_image_ref, width, height, 0, None)
            });
        });

        RwLock::into_inner(Arc::into_inner(orginal_image).unwrap())
    }
}

fn thread(
    s: &Scope,
    image: Arc<RwLock<RgbImage>>,
    width: u32,
    height: u32,
    y: u32,
    error_recv: Option<Receiver<[f32; 3]>>,
) {
    let mut next_pixel_error: [f32; 3] = [0.0; 3];

    // Ring buffer for the three pixels under us, to be sent to the next thread
    // errors_ring[ring_idx] will be the leftmost pixel
    let mut ring_idx = 0_usize;
    let mut errors_ring: [[f32; 3]; 3] = [[0.0; 3]; 3];
    let (next_error_send, next_error_recv) = unbounded::<[f32; 3]>();
    let mut next_error_recv_opt = Some(next_error_recv);

    for x in 0..width {
        if let Some(error_recv) = &error_recv {
            if let Ok(received_error) = error_recv.recv() {
                for i in 0..3 {
                    next_pixel_error[i] += received_error[i];
                }
            } else {
                // Don't care. The previous thread just ended, but we can continue with 0 error.
            }
        }

        let mut color;
        {
            // Get original pixel color from image
            let image = image.read();
            color = image.get_pixel(x, y).to_owned();

            // Apply dithering error from previous thread (the factor was precomputed)
            distribute_rgb_channels(&mut color, next_pixel_error, 1.0);
        }

        // Find the closest MC color
        let (closest_color, difference) = get_color_tree().find_closest(&color);

        // Apply converted pixel
        {
            let mut image = image.write();
            *image.get_pixel_mut(x, y) = closest_color;
        }

        let errors = difference.map(|err| err as f32 / 256.0);

        // Propagate errors and communicate with the next thread
        for i in 0..3 {
            next_pixel_error[i] = errors[i] * DITHERING_MATRIX[0].1;
            errors_ring[ring_idx][i] += errors[i] * DITHERING_MATRIX[1].1;
            errors_ring[(ring_idx + 1) % 3][i] += errors[i] * DITHERING_MATRIX[2].1;

            // The last one we can overwrite because it was sent over in the last iteration
            errors_ring[(ring_idx + 2) % 3][i] = errors[i] * DITHERING_MATRIX[3].1;
        }

        if x == 1 && next_error_recv_opt.is_some() && y < height - 1 {
            let image = image.clone();
            let next_error_recv_opt = next_error_recv_opt.take();
            s.spawn(move |s| {
                thread(s, image, width, height,y + 1, next_error_recv_opt)
            });
        }

        // Send the error
        next_error_send.send(errors_ring[ring_idx]).unwrap();

        ring_idx += 1;
        ring_idx %= 3;
    }

}