use std::fs;
use std::process;

pub fn creation(name: &str, lang: &str) {
    println!("Creating {} in {}", name, lang);
    if let Err(e) = fs::create_dir_all(name) {
        eprintln!("Cannot create directory {}: {}", name, e);
        process::exit(1);
    }

}