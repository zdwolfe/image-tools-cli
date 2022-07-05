#![allow(unused)]

use image::GenericImageView;
use image::imageops::FilterType;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    mode: String,
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
    output_filename: String
}

fn main() {
    let args = Cli::parse();
    if (args.mode != "youtube-video-thumbnail") {
        println!("Unsupported mode. Only supported mode is youtube-video-thumbnail");
        std::process::exit(1);
        return;
    }

    let img = image::open(&args.path).unwrap();
    let resized = img.resize(1280, 720, FilterType::Nearest);
    resized.save(&args.output_filename).unwrap();    
}
