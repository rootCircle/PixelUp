use image::DynamicImage;

pub fn apply_sepia_filter(img: &mut DynamicImage) -> DynamicImage {
    let mut img = img.to_rgb8();
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let (red, green, blue) = (pixel[0], pixel[1], pixel[2]);

            let red_out =
                ((red as f32 * 0.393) + (green as f32 * 0.769) + (blue as f32 * 0.189)) as u8;
            let green_out =
                ((red as f32 * 0.349) + (green as f32 * 0.686) + (blue as f32 * 0.168)) as u8;
            let blue_out =
                ((red as f32 * 0.272) + (green as f32 * 0.534) + (blue as f32 * 0.131)) as u8;

            img.put_pixel(x, y, image::Rgb([red_out, green_out, blue_out]));
        }
    }

    DynamicImage::ImageRgb8(img)
}
