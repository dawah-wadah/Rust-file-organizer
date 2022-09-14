use std::{env};

// use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap();

    println!("Searching for {}", config.query);
    println!("In file for {}", config.filename);
    println!("{:?}", args);
}

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
    if args.len() <= 1 {
        return Err("not enough params".to_string())
    }
        let query: String = args[1].clone();
        let filename: String = args[2].clone();
    
        return Ok(Config{query, filename})
    }

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