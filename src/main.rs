mod args;
mod backup;
mod paths;
mod utils;

use args::Args;
use backup::zip_files;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use paths::*;
use std::time::Duration;
use std::{
    fmt::format,
    os::unix::process,
    process::{Command, exit},
    time::Instant,
};
use utils::clear_screen;

use crate::utils::show_results;

fn main() {
    let args = Args::parse();
    let start_time = Instant::now();

    let src_path = get_src_path(&args).unwrap_or_else(|e| {
        eprintln!("ERROR: {}", e);
        exit(1);
    });

    let backup_name = get_folder_name(&src_path).unwrap_or_else(|e| {
        eprintln!("{}", e);
        exit(1);
    });

    let dst_path = get_dst_path(&args).unwrap_or_else(|e| {
        eprintln!("ERROR: {e}");
        exit(1);
    });

    let dst_path = format!("{}_{}.zip", dst_path, backup_name);
    let source_iter = create_folder_iterator(&src_path);
    let files = get_files_list(source_iter);
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::with_template("{spinner:.green} {msg}").unwrap());

    pb.enable_steady_tick(Duration::from_millis(40));
    if let Err(e) = zip_files(&files, &src_path, &dst_path, |file| {
        pb.set_message(format!("Compressing: {}", file));
    }) {
        pb.finish_and_clear();
        eprintln!("ERROR: {}", e);
        exit(1);
    }
    pb.finish_and_clear();

    clear_screen();
    let elapsed = start_time.elapsed();
    let finish_time = elapsed.as_secs_f64();
    show_results(&files, &src_path, &dst_path, finish_time);
}
