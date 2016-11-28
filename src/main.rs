extern crate image;

use std::env::{self, Args};
use std::collections::HashMap;
use std::thread::spawn;
use std::sync::{Arc, Mutex};

use image::{DynamicImage, GenericImage, Pixel, Rgb};

fn main() {
    let image = load_image(env::args()).unwrap();
    let color_map = Arc::new(Mutex::new(HashMap::new()));

    process_image_parallel(&image, &color_map);

    println!("{}", get_dominant_color(&(*color_map.lock().unwrap())));
}

fn load_image(mut arguments: Args) -> Result<DynamicImage, String> {
    arguments.nth(1)
        .ok_or("Please give at least one argument.".to_owned())
        .and_then(|path| image::open(path).map_err(|e| e.to_string()))
}

fn process_image_parallel(image: &DynamicImage, color_map: &Arc<Mutex<HashMap<Rgb<u8>, u64>>>) {
    const NTHREADS: usize = 8;
    let mut thread_handles = vec![];

    for _ in 0..NTHREADS {
        let color_map_clone = color_map.clone();
        let pixels = image.pixels().skip(NTHREADS * 1000).take(1000).collect();

        thread_handles.push(spawn(move || process_pixels(pixels, color_map_clone)));
    }

    for handle in thread_handles {
        handle.join().unwrap();
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

fn get_dominant_color(color_map: &HashMap<Rgb<u8>, u64>) -> String {
    match color_map.iter().max_by_key(|&(_, &count)| count) {
        Some((&color, _)) => rgb_to_hex(color),
        None => String::new(),
    }
}

fn rgb_to_hex(rgb: Rgb<u8>) -> String {
    format!("#{:X}{:X}{:X}", rgb.data[0], rgb.data[1], rgb.data[2])
}