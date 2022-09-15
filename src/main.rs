#![allow(unused)]
use readable_byte::readable_byte;
use std::env;
use std::error::Error;
use std::fs::{self, Metadata};
use std::os::unix::prelude::MetadataExt;
use std::str::FromStr;
use fs_extra::dir::get_size;
// use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap();

    println!("Searching for {}", config.path);
    println!("In file for {}", config.destination);
    let path_valid = is_path_redux(&config.path, "directory");
    let file_valid = is_path_redux(&config.path, "file");
    // let not_valid = is_path_redux(&config.path, "game");
    println!("{:?}", args);
    let path_valid_bool = path_valid.unwrap();
    println!(
        "Is this filepath leads to a directory: {:?}",
        path_valid_bool
    );
    println!(
        "Is this filepath leads to a file: {:?}",
        file_valid.unwrap()
    );

    if path_valid_bool {
        record_file(&config.path);
    }
    return;
}

struct Config {
    path: String,
    destination: String,
}

struct FileEntry {
    name: String,
    old_path: String,
    new_path: String,
    date_created: String,
    date_modified: String,
    date_moved: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() <= 1 {
            return Err("not enough params".to_string());
        }
        let path: String = args[1].clone();
        let destination: String = args[2].clone();

        return Ok(Config { path, destination });
    }
}

fn is_path_redux(filepath: &str, classifer: &str) -> std::io::Result<bool> {
    let attr = fs::metadata(filepath)?;
    // inspect attr ...
    match classifer {
        "directory" => return Ok(attr.is_dir()),
        "file" => return Ok(attr.is_file()),
        "symlink" => return Ok(attr.is_symlink()),
        _ => panic!("not a valid arguement"),
    }
    // inspect attr ...
}

fn record_file(filepath: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let current_dir = filepath;
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();
        let last_accessed = metadata.accessed()?.elapsed()?.as_secs();

        let file_size = readable_byte::b(metadata.len());
        if metadata.is_file() && false{
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?}, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                file_size,
                path.file_name().ok_or("No filename")?,
            );

            let filename = path.file_name().ok_or("No filename")?;
        }
        if metadata.is_dir() {
            println!(
                "Folder Path: {:?}, is read only: {:?}, size: {:?}",
                path.file_name().ok_or("No filename")?,
                metadata.permissions().readonly(),
                readable_byte::b(get_size(&path).unwrap()),
            );
            record_file(&path.into_os_string().into_string().unwrap());
        }
    }

    return Ok(true);
}

fn check_for_recent_changes(filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = filepath;
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );

    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed()?.as_secs();
        let last_accessed = metadata.accessed()?.elapsed()?.as_secs();

        if last_modified < 24 * 3600 && metadata.is_file() {
            println!(
                "Last modified: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?,
            );
        }
        if last_accessed < 24 * 3600 && metadata.is_file() {
            println!(
                "Last Accessed: {:?} seconds, is read only: {:?}, size: {:?} bytes, filename: {:?}",
                last_accessed,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename")?
            );
        }
    }

    Ok(())
}
