use image::DynamicImage;

pub fn apply_sepia_filter(img: &mut DynamicImage) -> DynamicImage {
    let mut img = img.to_rgb8();
    let (width, height) = img.dimensions();
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
            let r_out = ((r as f32 * 0.393) + (g as f32 * 0.769) + (b as f32 * 0.189)) as u8;
            let g_out = ((r as f32 * 0.349) + (g as f32 * 0.686) + (b as f32 * 0.168)) as u8;
            let b_out = ((r as f32 * 0.272) + (g as f32 * 0.534) + (b as f32 * 0.131)) as u8;
            img.put_pixel(x, y, image::Rgb([r_out, g_out, b_out]));
        }
    }

    DynamicImage::ImageRgb8(img)
}