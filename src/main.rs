#![allow(unused)]
use std::error::Error;
use std::{env};
use std::fs::{self, Metadata};


// use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap();

    println!("Searching for {}", config.path);
    println!("In file for {}", config.destination);
    let path_valid = is_path_redux(&config.path, "directory");
    let file_valid = is_path_redux(&config.path, "file");
    let not_valid = is_path_redux(&config.path, "game");
    println!("{:?}", args);
    println!("Is this filepath leads to a directory: {:?}", path_valid.unwrap());
    println!("Is this filepath leads to a file: {:?}", file_valid.unwrap());
}

struct Config {
    path: String,
    destination: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
    if args.len() <= 1 {
        return Err("not enough params".to_string())
    }
        let path: String = args[1].clone();
        let destination: String = args[2].clone();
    
        return Ok(Config{path, destination})
    }

}

fn is_path_redux(filepath: &str, classifer: &str) -> std::io::Result<bool> {
 let attr = fs::metadata(filepath)?;
    // inspect attr ...
    match classifer {
        "directory" => return Ok(attr.is_dir()),
        "file" => return Ok(attr.is_file()),
        "symlink" =>return Ok(attr.is_symlink()),
        _ => panic!("not a valid arguement")
    }
    // inspect attr ...
}

fn is_path(filepath: &str) -> std::io::Result<bool> {
 let attr = fs::metadata(filepath)?;
    // inspect attr ...
    Ok(attr.is_dir())
    // inspect attr ...
}
// fn mime(file_path: &str) {
//     match mime_type::File::new(Path::new(file_path)).get_mime_type() {
//         Err(e) => {
//             eprintln!("{}", e);
//             std::process::exit(exitcode::DATAERR);
//         }
//         Ok(m) => {
//             println!("{}", m)
//         }
//     };
// }