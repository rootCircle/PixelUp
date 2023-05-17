use clap::{Parser, ValueEnum};
use std::path::PathBuf;

const DEFAULT_IMAGE_OUTPUT_FILENAME: &str = "output.png";


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Filters 
{
    Sepia,
    Monocolor,
    Negative,
    Blur,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', long, required = true)]
    src: PathBuf,

    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(short, long,value_enum, required = true)]
    filter: Filters
}

fn main() {
    let mut args = Args::parse();

    args.output = match args.output {
        Some(x) => Some(x),
        None => Some(PathBuf::from(DEFAULT_IMAGE_OUTPUT_FILENAME)),
    };
    
    println!("{:#?}", args);
    
}