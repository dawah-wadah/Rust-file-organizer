use chrono::offset::Utc;
use readable_byte::readable_byte;
use std::ffi::OsStr;
use std::time::SystemTime;
use std::{error::Error, fs, io};

use chrono::format::ParseError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};

struct FileEntry {
    name: String,
    old_path: String,
    new_path: String,
    date_created: String,
    date_modified: String,
    date_moved: String,
}

impl FileEntry {
    fn new(
        name: &str,
        old_path: &str,
        new_path: &str,
        date_created: &str,
        date_modified: &str,
        date_moved: &str,
    ) -> Result<FileEntry, String> {
        return Ok(FileEntry {
            name: String::from(name),
            old_path: String::from(old_path),
            new_path: String::from(new_path),
            date_created: String::from(date_created),
            date_modified: String::from(date_modified),
            date_moved: String::from(date_moved),
        });
    }

    fn from_path(file_path: &str) -> Result<Vec<FileEntry>, Box<dyn Error>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(file_path)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = path.file_name().ok_or("No filename")?.to_str().unwrap();

            let metadata = fs::metadata(&path)?;
            let last_modified = metadata.modified()?.elapsed()?.as_secs();
            let last_accessed = metadata.accessed()?.elapsed()?.as_secs();

            if metadata.is_file() && false {
                println!(
                    "Last modified: {:?} seconds, is read only: {:?}, size: {:?}, filename: {:?}",
                    last_modified,
                    metadata.permissions().readonly(),
                    readable_byte::b(metadata.len()),
                    path.file_name().ok_or("No filename")?,
                );

                let dt: DateTime<Utc> = DateTime::from(metadata.created().unwrap());
                let modified: DateTime<Utc> = DateTime::from(metadata.modified().unwrap());
                let accessed: DateTime<Utc> = DateTime::from(metadata.accessed().unwrap());
                let entry_line = FileEntry::new(
                    file_name.into(),
                    path.to_str().unwrap(),
                    path.to_str().unwrap(),
                    dt.to_string().as_str(),
                    modified.to_string().as_str(),
                    accessed.to_string().as_str(),
                )
                .unwrap();

                let filename = path.file_name().ok_or("No filename")?;
                entries.push(entry_line);
            }
        }
        return Ok(entries);
    }
}
