extern crate image;

use std::fs::File;
use std::path::Path;
use std::char;
use std::u32;

use image::GenericImage;

fn add_num(s:&mut String, l:char) {
    if l == '-' {
        return;
    }
    s.push(l);
    if s.len() >=2 {
        let mut num_again = u32::from_str_radix(&s, 16).unwrap();
        print!("{}", char::from_u32(num_again as u32).unwrap());
        s.clear();
    }
}

fn main() {
    // Use the open function to load an image from a Path.
    // ```open``` returns a dynamic image.
    let img = image::open(&Path::new("puzzle.png")).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", img.color());
    let mut p = img.get_pixel(0,0);
    let mut count: usize = 1;
    let mut character = String::new();

    for pixel in img.pixels() {
        if p != pixel.2 {
            let tmp = char::from_u32(count as u32).unwrap();
            add_num(&mut character, tmp);
            count = 0;
        }
        p = pixel.2;
        count += 1;
    }
}
