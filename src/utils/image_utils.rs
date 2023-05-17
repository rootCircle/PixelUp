use colored::Colorize;
use image::DynamicImage;
use std::PathBuf;

pub fn save_image(img: DynamicImage, output_file: PathBuf) -> bool {
    match img.save(output_file) {
        Ok(_) => true,
        Err(err) => {
            eprintln!("{}", "Some Error Occured Saving file!".red());
            eprintln!("{}", err);
            false
        }
    }
}