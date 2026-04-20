use chrono::prelude::*;
use shellexpand;
use std::env::{self, args};
use std::path::Path;
use walkdir::{IntoIter, WalkDir};

use crate::args::Args;

pub fn create_folder_iterator(folder: &str) -> IntoIter {
    WalkDir::new(folder).into_iter()
}

pub fn get_folder_name(path: &str) -> Result<String, String> {
    let expanded = shellexpand::tilde(path);
    dbg!(&expanded);
    let os_path = Path::new(expanded.as_ref());
    if !os_path.is_dir() {
        dbg!(os_path);
        return Err(format!("'{}' is not a valid directory", path));
    }

    if os_path.is_relative() {
        Ok(path.to_string())
    } else {
        Ok(os_path.file_name().unwrap().to_str().unwrap().to_string())
    }
}

pub fn get_files_list(folder_iter: IntoIter) -> Vec<String> {
    let excluded = [
        "obj", "out", "pdb", "lib", "cache", "pyc", "pyo", "class", "log", "pack", "d", "o",
        "rlib", "rmeta", "h", "sample", "so", "bin", "a",
    ];

    let mut files = Vec::new();

    for entry in folder_iter.filter_map(|e| e.ok()) {
        if let Some(p) = entry.path().to_str() {
            let path = entry.path();

            let is_excluded = path
                .extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| excluded.contains(&e));

            let is_none = path.extension().is_none();

            if !is_excluded && !is_none {
                files.push(p.to_string());
            }
        }
    }

    files
}

pub fn get_src_path(args: &Args) -> Result<String, String> {
    if args.is_nvim {
        return Ok(format!("{}", shellexpand::tilde("~/.config/nvim")));
    }

    if let Some(value) = &args.src_directory {
        let path = value;
        let resolved = env::current_dir()
            .map_err(|e| format!("Failed to get current directory: {e}"))?
            .join(&path);

        if resolved.is_dir() {
            Ok(resolved.to_str().unwrap().to_string())
        } else {
            Err(format!("'{}' is not a valid directory", path))
        }
    } else {
        Err(format!("The path has to be indicated."))
    }
}

pub fn get_dst_path(args: &Args) -> Result<String, String> {
    let now = Local::now();
    let datetime = now.format("%y-%m-%d_%H-%M").to_string();

    if let Some(value) = &args.dst_directory {
        let path = Path::new(value);

        if !path.is_dir() {
            return Err(format!("'{}' is not a valid directory", value));
        }

        let absolute_path = if path.is_relative() {
            env::current_dir()
                .map_err(|e| format!("Failed to get current directory: {e}"))?
                .join(path)
                .to_str()
                .unwrap()
                .to_string()
        } else {
            value.to_string()
        };

        Ok(format!("{}/{}", absolute_path, datetime))
    } else {
        Ok(format!("/home/jayfaza/backup/{}", datetime))
    }
}
