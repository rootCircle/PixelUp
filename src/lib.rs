use std::{path::PathBuf,process};
use colored::Colorize;
use clap::{Parser, ValueEnum};
use image::{DynamicImage, io::Reader as ImageReader};
mod utils;
mod photo_filters;
use crate::photo_filters::img_sepia::apply_sepia_filter;
use crate::photo_filters::img_grayscale::apply_grayscale;
use crate::utils::image_util::save_image;
// use pixel_up::utils;

const DEFAULT_IMAGE_OUTPUT_FILENAME: &str = "output.png";



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'i', long, required = true)]
    src: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, long,value_enum, required = true)]
    filter: Filters
}


impl Args {
    pub fn new() -> Self {
        Self::parse()
    }

    // It will validate the fields of Args Struct and return Result enum containing opened Dynamic Image
    pub fn check_and_open(&mut self) -> Result<DynamicImage, image::ImageError>{
        
        // Check if destination file with same name exists or not, for clash
        // This check is done only in case, if output filename is specified
        // Else it will populate, it with default value
        self.check_destination_file();

        // Check if source file exists or not
        self.check_source_file();
        
        // Checks Image Type and returns Dynamic Image
        self.check_and_open_image()
        
    }

    fn check_destination_file(&mut self) {
        self.output = match &self.output {
            Some(x) => {
                if x.exists() {
                    eprintln!("{}", "Destination File with same name exists!".yellow().bold());
                    process::exit(1);
                }
                Some(x.to_path_buf())
            },
            None => Some(PathBuf::from(DEFAULT_IMAGE_OUTPUT_FILENAME)),
        };
    }

    fn check_source_file(&mut self) {   
        if ! self.src.exists() {
            eprintln!("{}", "Source File does not exists!".red().bold());
            process::exit(1);
        }
    }

    fn check_and_open_image(&mut self) -> Result<DynamicImage, image::ImageError> {
        let img_result = ImageReader::open(&self.src).unwrap().decode();
            if let Err(ref err) = img_result {
                println!("Error decoding image: {}", err);
                process::exit(2);
            }
        img_result
    }

    pub fn get_filter(&mut self) -> Filters {
        self.filter
    }

    pub fn get_output(&mut self) -> PathBuf {
        match &self.output {
            Some(x) => x.to_path_buf(),
            None => PathBuf::new() 
        }
    }

    pub fn apply_filters(&mut self, mut img: DynamicImage) {
        let img = match self.get_filter() {
            // Filters::Blur => {},
            Filters::Grayscale => {
                apply_grayscale(&mut img)
            },
            Filters::Sepia => {
                apply_sepia_filter(&mut img)
            },
            Filters::Negative | Filters::Blur => todo!()
            // Filters::Negative => {},
        };

        save_image(img, self.get_output());

        // pixel_up::save_image()
    }

}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Filters 
{
    Sepia,
    Grayscale,
    Negative,
    Blur,
}
