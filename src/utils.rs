use clearscreen;
use colored::Colorize;
use std::fs::metadata;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn clear_screen() {
    clearscreen::clear().unwrap();
}

pub fn show_results(files: &Vec<String>, src: &str, dst: &str, time: f64) {
    let files_quantity = files.len().to_string();
    let size_in_bytes = metadata(dst).unwrap().len();
    let size_in_mbytes = bytes_to_mb(size_in_bytes);
    println!("{}", "✓ Done!".green().bold());
    println!(
        "  {} {} {}",
        "Just in".to_string().green().italic().bold(),
        time.to_string().green().italic().bold().underline(),
        "seconds".to_string().green().italic().bold()
    );
    println!(
        "  {}: {}",
        "Files".to_string().green().bold(),
        files_quantity.green().bold().underline()
    );
    println!(
        "  {}: {}",
        "Input".to_string().green().bold(),
        src.to_string().green().bold()
    );
    println!(
        "  {}: {}",
        "Output".to_string().green().bold(),
        dst.to_string().green().bold()
    );
    println!(
        "  {}: {:.4} MB",
        "Final weight".to_string().green().bold(),
        size_in_mbytes.to_string().green().bold()
    );
}

fn bytes_to_mb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0)
}
