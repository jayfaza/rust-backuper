use chrono::prelude::*;
use std::env;
use std::path::Path;
use walkdir::{IntoIter, WalkDir};

use crate::args::Args;

pub fn create_folder_iterator(folder: &str) -> IntoIter {
    WalkDir::new(folder).into_iter()
}

pub fn get_folder_name(path: &str) -> Result<String, String> {
    let expanded = shellexpand::tilde(path);
    let os_path = Path::new(expanded.as_ref());
    if !os_path.is_dir() {
        return Err(format!("'{}' is not a valid directory", path));
    }

    Ok(os_path.file_name().unwrap().to_str().unwrap().to_string())
}

pub fn get_files_list(folder_iter: IntoIter) -> Vec<String> {
    let excluded_dirs = [
        "target",
        "__pycache__",
        ".pytest_cache",
        ".mypy_cache",
        ".ruff_cache",
        "node_modules",
        ".next",
        ".nuxt",
        ".turbo",
        "dist",
        ".parcel-cache",
        "build",
        ".gradle",
        ".git",
        ".cache",
        ".tmp",
        "tmp",
        ".venv",
        "venv",
    ];

    let excluded_exts = [
        "o", "obj", "a", "so", "dylib", "dll", "lib", "pdb", "rlib", "rmeta", "d", "class", "pyc",
        "pyo", "sample",
    ];

    let mut files = Vec::new();

    for entry in folder_iter.filter_map(|e| e.ok()) {
        let path = entry.path();

        let in_excluded_dir = path.components().any(|c| {
            c.as_os_str()
                .to_str()
                .is_some_and(|s| excluded_dirs.contains(&s))
        });

        if in_excluded_dir {
            continue;
        }

        let has_excluded_ext = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| excluded_exts.contains(&e));

        if has_excluded_ext {
            continue;
        }

        if path.is_file() {
            if let Some(p) = path.to_str() {
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
        Ok(format!(
            "{}/{}",
            shellexpand::tilde("~/backup").as_ref(),
            datetime
        ))
    }
}
