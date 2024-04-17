use clap::Parser;
use ndarray;
use std::path::Path;
use video_rs::Decoder;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input video to be compressed
    #[arg(short, long)]
    video_path: String,

    /// Output file path
    #[arg(short, long)]
    output_path: String,
}

fn main() {
    let args = Args::parse();
    let file_path = Path::new(&args.video_path);

    if !file_path.is_file() {
        // WARN: MAybe not exit the prorgam incase the input is not valid
        panic!("The path to the input video is invalid");
    }
    // TODO: Handle exit program incase the file is not a video
    //
    //
    let mut decoder = Decoder::new(file_path).expect("The provided file is not a video");

    for frame in decoder.decode_raw_iter() {
        if let Ok(frame) = frame {
            println!("{:?}", frame.color_space());
        }
    }
}
