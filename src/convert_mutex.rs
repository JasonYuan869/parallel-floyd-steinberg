use std::cmp::Ordering;
use std::ops::Deref;
use crate::colors::get_color_tree;
use crate::convert::{Converter, distribute_rgb_channels, DITHERING_MATRIX};
use crossbeam::channel::{Receiver, Sender, unbounded};
use image::{Rgb, RgbImage};
use rayon::Scope;
use std::sync::{Arc, Mutex};

/// A converter that implements the Floyd-Steinberg dithering algorithm using multiple threads.
/// To access the pixels in a thread-safe manner, it represents the image as a vector of Arc<Mutex<Rgb<u8>>>.
pub struct MutexConverter;

impl MutexConverter {
    pub fn new() -> Self {
        MutexConverter {}
    }
}

impl Converter for MutexConverter {
    fn convert(&self, mut image: RgbImage) -> RgbImage {
        let (width, height) = image.dimensions();

        // Thread safe image
        let mut image_send: Vec<Arc<Mutex<Rgb<u8>>>> = Vec::with_capacity((width * height) as usize);

        // Initialize each pixel
        for i in 0..(image.height() * image.width()) {
            let x = i % image.width();
            let y = i / image.width();

            image_send.push(Arc::new(Mutex::new(*image.get_pixel(x, y))));
        }

        // Wrap in Arc
        let image_send = Arc::new(image_send);

        // Convert the image
        rayon::scope(|s| {
            thread(
                s,
                None,
                0,
                width,
                height,
                image_send.clone(),
            )
        });

        // Get rid of the mutexes and return the image
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize;
                let color = image_send[index].lock().unwrap();
                image.put_pixel(x, y, *color);
            }
        }

        image
    }
}

fn thread(
    s: &Scope,
    ch: Option<Receiver<()>>,
    y: u32,
    width: u32,
    height: u32,
    image: Arc<Vec<Arc<Mutex<Rgb<u8>>>>>,
) {
    let mut sender: Option<Sender<()>> = None;
    for x in 0..width {
        let index = (y * width + x) as usize;

        // Block until message received, unless this is the first row
        // This is to ensure that the threads are in sync
        if let Some(ch) = &ch {
            // Don't care if it errors: just unblock
            let _ = ch.recv();
        }

        // Scope to retrieve the pixel value
        let closest_color: Rgb<u8>;
        let difference: [i16; 3];
        {
            let color = image[index].lock().unwrap();
            (closest_color, difference) = get_color_tree().find_closest(color.deref());
        }

        // Apply converted pixel
        {
            let mut pixel = image[index].lock().unwrap();
            *pixel = closest_color;
        }

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

            // Scope to propagate pixel errors
            {
                let mut original_color = image[(y as u32 * width + x as u32) as usize]
                    .lock()
                    .unwrap();
                distribute_rgb_channels(&mut original_color, errors, factor);
            }
        }

        if y < height - 1 {
            match x.cmp(&1) {
                // This whole block will only be triggered once but the borrow checker doesn't know
                // So all the `move` shenanigans is to satisfy the borrow checker
                Ordering::Equal => {
                    let (_sender, receiver) = unbounded::<()>();
                    sender = Some(_sender);

                    // Spawn the next thread with moved values
                    let cloned_image_ref = image.clone();
                    s.spawn(move |s1| {
                        thread(
                            s1,
                            Some(receiver),
                            y + 1,
                            width,
                            height,
                            cloned_image_ref,
                        )
                    });
                }
                Ordering::Greater => {
                    // Send a message down the channel
                    if let Some(sender) = &sender {
                        sender.send(()).unwrap();
                    }
                }
                _ => {}
            }
        }
    }
}
