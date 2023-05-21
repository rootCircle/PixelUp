use image::DynamicImage;

pub fn apply_negative(img: &mut DynamicImage) -> DynamicImage {
    let mut img = img.to_rgb8();
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let (red, green, blue) = (255 - pixel[0], 255 - pixel[1], 255 - pixel[2]);

            img.put_pixel(x, y, image::Rgb([red, green, blue]));
        }
    }

    DynamicImage::ImageRgb8(img)
}
