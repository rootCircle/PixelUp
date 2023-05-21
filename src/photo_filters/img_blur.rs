use image::DynamicImage;

const BLUR_STRETCH: u32 = 1;

pub fn apply_blur(img: &mut DynamicImage) -> DynamicImage {
    let mut img = img.to_rgb8();
    let (width, height) = img.dimensions();

    for x in 0..width {
        for y in 0..height {
            // Store sum and count of neighbouring pixels
            let mut r_sum:u32  = 0;
            let mut g_sum:u32 = 0;
            let mut b_sum:u32 = 0;
            let mut c: u32 = 0;

            // Max Distance in Gaussian Kernel from central pixel in a defined straight direction

            let blur_stretch = std::cmp::min(std::cmp::min(height / 2, width / 2), BLUR_STRETCH);

            // Set of conditions to prevent overflow in extremes sides while averaging
            let row_start: u32 = if y  >= blur_stretch { 
                y - blur_stretch
            } 
            else {
                0
            };

            let row_end = if y + blur_stretch < height {
                y + blur_stretch
            } else {
                height - blur_stretch
            };

            let col_start = if x >= blur_stretch {
                x - blur_stretch
            } else {
                0
            };

            let col_end = if x + blur_stretch < width {
                x + blur_stretch
            } else {
                width - blur_stretch
            };


            // Take sum of RGB values of the specified Gaussian Kernel
            for i in row_start..(row_end + 1) {
                for j in col_start..(col_end  + 1) {
                
                    let n_pixel = img.get_pixel(j, i);
                    r_sum += n_pixel[0] as u32;
                    g_sum += n_pixel[1] as u32;
                    b_sum += n_pixel[2] as u32;

                    c += 1;
                }
            }

            img.put_pixel(x, y, image::Rgb([
                (r_sum as f64 / c as f64).round() as u8,
                (g_sum as f64 / c as f64).round() as u8,
                (b_sum as f64 / c as f64).round() as u8]));
        }
    }

    DynamicImage::ImageRgb8(img)
}
