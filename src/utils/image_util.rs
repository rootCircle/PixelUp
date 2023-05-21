use colored::Colorize;
use image::DynamicImage;
use std::path::PathBuf;

pub fn save_image(img: &DynamicImage, output_file: &PathBuf) -> bool {
    match img.save(output_file) {
        Ok(_) => {
            println!(
                "{} at {}",
                "Image saved successfully".green().bold(),
                output_file.to_string_lossy().bright_cyan()
            );
            true
        }
        Err(err) => {
            eprintln!("{}", "Some Error Occured Saving file!".red());
            eprintln!("{}", err);
            false
        }
    }
}
