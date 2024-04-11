use image::{DynamicImage, Rgba, RgbaImage};
use shellexpand::tilde;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "Please specify a path to an image. Usage: {} <input_image_path>",
            args[0]
        );
        std::process::exit(1);
    }

    // Specify the input and output file paths
    let input_path = &args[1];
    let fallback_output_dir = tilde(".").to_string();
    let output_dir = match args.get(2) {
        Some(output_arg) => output_arg,
        None => &fallback_output_dir,
    };

    // Load the image from the input path
    let img = image::open(input_path).expect("Failed to open image");

    let negative_thumbnail_output_path = format!("{}/thumbnail-negative.png", output_dir);
    let thumbnail_output_path = format!("{}/thumbnail.png", output_dir);

    // Process the image (adjust exposure, contrast, and convert to RGBA)
    let (black_img, white_img) = process_image(&img);

    // Save the processed images to the output path
    black_img
        .save(&negative_thumbnail_output_path)
        .expect("Failed to save image");

    white_img
        .save(&thumbnail_output_path)
        .expect("Failed to save image");

    println!(
        "Images saved to: {} & {}",
        negative_thumbnail_output_path, thumbnail_output_path
    );
}

fn process_image(input_image: &DynamicImage) -> (RgbaImage, RgbaImage) {
    // Adjust exposure and contrast
    let adjusted_img = input_image.brighten(70).adjust_contrast(25.0);

    // Convert to RGBA format
    let rgba_img = adjusted_img.to_rgba8();

    // Create a new RGBA image with white converted to transparency
    // and non-transparent pixels set to black or white, respectively
    let mut black_img = RgbaImage::new(rgba_img.width(), rgba_img.height());
    let mut white_img = RgbaImage::new(rgba_img.width(), rgba_img.height());

    for (x, y, pixel) in rgba_img.enumerate_pixels() {
        let mut black_pixel = *pixel;
        let mut white_pixel = *pixel;

        set_white_pixels_transparent(&mut black_pixel, 0);
        set_white_pixels_transparent(&mut white_pixel, 255);

        black_img.put_pixel(x, y, black_pixel);
        white_img.put_pixel(x, y, white_pixel);
    }

    (black_img, white_img)
}

fn set_white_pixels_transparent(pixel: &mut Rgba<u8>, color: u8) {
    if pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255 {
        // Set white pixels to transparent
        pixel[3] = 0;
    } else {
        // Set colors for non-transparent pixels
        pixel[0] = color;
        pixel[1] = color;
        pixel[2] = color;
    }
}
