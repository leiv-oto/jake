use std::fs;
use std::path::Path;
use std::process;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Lang {
    Kotlin,
    Java,
}

pub fn creation(name: &str, lang: Lang, mixed: bool) {
    println!("Creating {} in {:?}{}", name, lang, if mixed {" (mixed)"} else {""});

    let base_path = Path::new(name);
    let src_path = base_path.join("src");

    if mixed {
        let kt_dir = src_path.join("kotlin");
        let java_dir = src_path.join("java");

        if let Err(e) = fs::create_dir_all(&kt_dir) {
            eprintln!("Cannot create directory {}: {}", kt_dir.display(), e);
            process::exit(1);
        }
        if let Err(e) = fs::create_dir_all(&java_dir) {
            eprintln!("Cannot create directory {}: {}", java_dir.display(), e);
            process::exit(1);
        }


    } else {

    }
}