use image::DynamicImage;

pub fn apply_edge(img: &mut DynamicImage) -> DynamicImage {
    
    let img = img.to_rgb8();
    
    // To store new computed image
    let mut new_img = img.clone();
    
    let (width, height) = img.dimensions();

    let width = width as i32;
    let height = height as i32;

    for y in 0..height {
        for x in 0..width {

            // For Gx
            let mut r_sum_x: i32 = 0;
            let mut g_sum_x: i32 = 0;
            let mut b_sum_x: i32 = 0;

            // For Gy
            let mut r_sum_y: i32 = 0;
            let mut g_sum_y: i32 = 0;
            let mut b_sum_y: i32 = 0;

            let stretch = 1;

            // Set of conditions to prevent overflow in extremes sides while averaging
            let row_start = if y >= stretch {
                y - stretch
            } else {
                0
            };

            let row_end = if y + stretch < height {
                y + stretch
            } else {
                height - stretch
            };

            let col_start = if x >= stretch {
                x - stretch
            } else {
                0
            };

            let col_end = if x + stretch < width {
                x + stretch
            } else {
                width - stretch
            };
            
            for k in row_start..(row_end + 1) {
                for l in col_start..(col_end + 1) {
                    
                    let pixel = img.get_pixel(l as u32, k as u32);
                    let (red, green, blue) = (pixel[0], pixel[1], pixel[2]);

                    if k != y {
                        r_sum_x += (l - x) * red as i32;
                        b_sum_x += (l - x) * blue as i32;
                        g_sum_x += (l - x) * green as i32;
                    }
                    else {
                        r_sum_x += 2 * (l - x) * red as i32;
                        b_sum_x += 2 * (l - x) * blue as i32;
                        g_sum_x += 2 * (l - x) * green as i32;
                    }

                    if l != x {
                        r_sum_y += (k - y) * red as i32;
                        b_sum_y += (k - y) * blue as i32;
                        g_sum_y += (k - y) * green as i32;
                    }
                    else {
                        r_sum_y += 2 * (k - y) * red as i32;
                        b_sum_y += 2 * (k - y) * blue as i32;
                        g_sum_y += 2 * (k - y) * green as i32;
                    }
                }
            }

            let red_avg = ((r_sum_x * r_sum_x + r_sum_y * r_sum_y) as f32).sqrt().round() as u8;
            let green_avg = ((g_sum_x * g_sum_x + g_sum_y * g_sum_y) as f32).sqrt().round() as u8;
            let blue_avg = ((b_sum_x * b_sum_x + b_sum_y * b_sum_y) as f32).sqrt().round() as u8;


            new_img.put_pixel(
                x as u32,
                y as u32,
                image::Rgb([
                    red_avg, green_avg, blue_avg
                ]),
            );
        }
    }

    DynamicImage::ImageRgb8(new_img)
}
