extern crate image;

use std::env::{self, Args};
use std::collections::HashMap;

use image::{DynamicImage, GenericImage, Pixel, Rgb};

fn main() {
    let image = load_image(env::args()).unwrap();
    let pixels = load_pixels(&image);
    let dominating_rgb = get_dominating_rgb(pixels).unwrap();

    println!("{}", rgb_to_hex(dominating_rgb));
}

fn load_image(mut arguments: Args) -> Result<DynamicImage, String> {
    arguments.nth(1)
        .ok_or("Please give at least one argument.".to_owned())
        .and_then(|path| image::open(path).map_err(|e| e.to_string()))
}

fn load_pixels(image: &DynamicImage) -> Vec<Rgb<u8>> {
    let mut result = Vec::new();

    for x in 0..image.width() {
        for y in 0..image.height() {
            result.push(image.get_pixel(x, y).to_rgb());
        }
    }

    result
}

fn get_dominating_rgb(pixels: Vec<Rgb<u8>>) -> Option<Rgb<u8>> {
    let mut color_map = HashMap::new();

    for pixel in pixels {
        let counter = color_map.entry(pixel).or_insert(0);
        *counter += 1;
    }

    match color_map.iter().max_by_key(|&(_, &count)| count) {
        Some((&color, _)) => Some(color),
        None => None,
    }
}

fn rgb_to_hex(rgb: Rgb<u8>) -> String {
    format!("#{:X}{:X}{:X}", rgb.data[0], rgb.data[1], rgb.data[2])
}