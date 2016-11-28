extern crate image;

use std::env::{self, Args};
use std::collections::HashMap;
use std::thread::spawn;
use std::sync::{Arc, Mutex};

use image::{DynamicImage, GenericImage, Pixel, Rgb};

fn main() {
    let image = load_image(env::args()).unwrap();
    let dominant_color = process_image_parallel(&image);

    println!("{}", dominant_color);
}

fn load_image(mut arguments: Args) -> Result<DynamicImage, String> {
    arguments.nth(1)
        .ok_or("Please give at least one argument.".to_owned())
        .and_then(|path| image::open(path).map_err(|e| e.to_string()))
}

fn process_image_parallel(image: &DynamicImage) -> String {
    const NTHREADS: usize = 8;
    let mut thread_handles = vec![];

    let color_map: Arc<Mutex<HashMap<Rgb<u8>, u64>>> = Arc::new(Mutex::new(HashMap::new()));

    for _ in 0..NTHREADS {
        let color_map_clone = color_map.clone();
        let pixels = image.pixels().skip(NTHREADS * 1000).take(1000).collect();

        thread_handles.push(spawn(move || process_pixels(pixels, color_map_clone)));
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    let data = color_map.lock().unwrap();

    match data.iter().max_by_key(|&(_, &count)| count) {
        Some((&color, _)) => format!("#{:X}{:X}{:X}", color.data[0], color.data[1], color.data[2]),
        None => String::new(),
    }
}

fn process_pixels(pixels: Vec<(u32, u32, image::Rgba<u8>)>,
                  color_map: Arc<Mutex<HashMap<Rgb<u8>, u64>>>) {
    for (_, _, pixel) in pixels {
        let pixel_rgb = pixel.to_rgb();
        let mut data = color_map.lock().unwrap();

        let counter = data.entry(pixel_rgb).or_insert(0);
        *counter += 1;
    }
}