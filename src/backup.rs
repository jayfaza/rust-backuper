use std::fs::File;
use std::fs::read;
use std::io::{self, Write};
use std::path::Path;
use zip::write::{SimpleFileOptions, ZipWriter};

pub fn zip_files<F>(
    files_list: &[String],
    src_dir_path: &str,
    dst_dir_path: &str,
    mut on_progress: F,
) -> io::Result<()>
where
    F: FnMut(&str),
{
    let source = Path::new(src_dir_path);
    let output_file = File::create(Path::new(dst_dir_path))?;
    let mut zip_writer = ZipWriter::new(output_file);
    let archive_options = SimpleFileOptions::default();

    for path in files_list {
        let os_path = Path::new(path);
        let relative_path = os_path
            .strip_prefix(source)
            .unwrap_or(os_path)
            .to_str()
            .unwrap();
        on_progress(relative_path);
        if os_path.is_dir() {
            zip_writer.add_directory(relative_path, archive_options)?;
        } else {
            zip_writer.start_file(relative_path, archive_options)?;
            let content = read(os_path)?;
            zip_writer.write_all(&content)?;
        }
    }
    zip_writer.finish()?;

    Ok(())
}
