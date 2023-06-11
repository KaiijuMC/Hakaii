mod hakaii;

use std::fs;
use std::env;
use std::process;

use clap::Parser;
use fs_extra::dir::get_size;

use hakaii::clean_regions;

#[derive(Parser)]
#[command(name = "Hakaii")]
#[command(author = "Kugge <sofiane.djerbi38@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "Filter chunks based on Inhabited Time NBT.", long_about = None)]
struct Args {
    /// World directory
    #[arg(value_name = "FOLDER")]
    world_dir: String,
    /// Minimum inhabited time
    #[arg(value_name = "TICKS")]
    min_inhabited_time: i64,
    /// Number of threads
    #[arg(value_name = "THREADS")]
    #[arg(short, long, default_value_t = 1)]
    threads: u32,
    /// ZSTD Compression level
    #[arg(value_name = "COMPRESSION LEVEL")]
    #[arg(short, long, default_value_t = 3)]
    compression_level: i32,
}

fn format_size(bytes: u64) -> String {
    if bytes == 0 {
        return String::from("0 B");
    }

    let suffixes = ["B", "KB", "MB", "GB", "TB"];
    let base: u64 = 1024;
    let exponent = ((bytes as f64).log(base as f64)).floor() as u32;
    let value = bytes as f64 / (base.pow(exponent) as f64);
    let formatted_value = format!("{:.2}", value);

    format!("{} {}", formatted_value, suffixes[exponent as usize])
}

fn main() {
    let args: Args = Args::parse();

    let duration = args.min_inhabited_time;
    let threads = args.threads;
    let compression_level = args.compression_level;

    println!(
        "We will delete chunks inhabited less than {} secs / {} ticks.",
        duration as f32 / 20.0,
        duration
    );

    let dirname = &args.world_dir;

    let mut file_names: Vec<String> = Vec::new();
    let entries = fs::read_dir(format!("{}/region", dirname))
        .expect("Failed to read directory");
    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            if let Some(name) = file_name.to_str() {
                file_names.push(String::from(name));
            }
        }
    }

    let size_before: u64 = get_size(dirname).unwrap();
    clean_regions(dirname, duration, compression_level, file_names);
    let size_after: u64 = get_size(dirname).unwrap();
    
    println!(
        "Reduced size from {} to {}",
        format_size(size_before),
        format_size(size_after)
    );
}
