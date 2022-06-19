use crossterm::{
    queue,
    style::{Color, Print},
    style::{ResetColor, SetBackgroundColor},
};
use image::{imageops::resize, DynamicImage, ImageBuffer, ImageError, Rgba};
use std::io::{stdout, Write};
static VERSION: &'static str = "1.0";

fn help() {
    println!(
        "Image Converter v{}

  Usage:
    ./ImageConverter [IMAGE]

  Options:
    IMAGE -- Any image file",
        VERSION
    );
}

fn real_main(image_file: &str) {
    println!("Loading image..");
    let image: Result<DynamicImage, ImageError> = image::open(&image_file); // Load the image into memory
    let image = match image {
        // Check if the image was loaded correctly
        Ok(img) => img,
        Err(error) => {
            println!("Could not open the file, error: {}", error);
            std::process::exit(1);
        }
    };
    println!("Resizing image..");
    let terminal_size = term_size::dimensions(); // Get terminal size
    let terminal_size = match terminal_size {
        // Check if terminal size is valid
        Some(size) => size,
        None => {
            println!("Could not get terminal size");
            std::process::exit(1);
        }
    };

    let terminal_width = terminal_size.0;
    let terminal_height = terminal_size.1;

    let resized_image: ImageBuffer<Rgba<u8>, Vec<u8>> = resize(
        // Resize the image to the terminal size
        &image,
        terminal_width as u32,
        terminal_height as u32,
        image::imageops::FilterType::Nearest,
    );

    let rgb_image = DynamicImage::ImageRgba8(resized_image).to_rgb8(); // Convert the image to RGB so we dont have any transparency issues
    let mut stdout = stdout();

    for pixel in rgb_image.pixels() {
        // Iterate over pixels and gather their RGB values
        match queue!(
            stdout,
            SetBackgroundColor(Color::Rgb {
                r: pixel[0],
                g: pixel[1],
                b: pixel[2]
            }),
            Print(" "),
            ResetColor
        ) {
            // Print the pixel to the terminal
            Ok(_) => (),
            Err(error) => {
                println!("Could not print to terminal, error: {}", error);
                std::process::exit(1);
            }
        };
    }

    match stdout.flush() {
        // Print everything in the queue to the terminal
        Ok(_) => (),
        Err(error) => {
            println!("Could not flush terminal, error: {}", error);
            std::process::exit(1);
        }
    };
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        2 => {
            real_main(&args[1][..]);
        }
        _ => {
            help();
        }
    }
}
