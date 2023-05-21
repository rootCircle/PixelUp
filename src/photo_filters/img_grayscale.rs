use image::DynamicImage;

pub fn apply_grayscale(img: &mut DynamicImage) -> DynamicImage {
    let mut img = img.to_rgb8();
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let (red, green, blue) = (pixel[0], pixel[1], pixel[2]);
            let mut avg = (red as u32 + green as u32 + blue as u32) / 3;

            if avg > 255 {
                avg = 255;
            }

            let avg = avg as u8;
            img.put_pixel(x, y, image::Rgb([avg, avg, avg]));
        }
    }

    DynamicImage::ImageRgb8(img)
}
