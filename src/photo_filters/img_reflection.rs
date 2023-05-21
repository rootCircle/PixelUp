use image::DynamicImage;

pub fn apply_reflection(img: &mut DynamicImage) -> DynamicImage {
    let mut img = img.to_rgb8();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..(width / 2) {
            let pixel = *img.get_pixel(x, y);
            let r_pixel = *img.get_pixel(width - x - 1, y);

            img.put_pixel(x, y, r_pixel);
            img.put_pixel(width - x - 1, y, pixel);
        }
    }

    DynamicImage::ImageRgb8(img)
}
