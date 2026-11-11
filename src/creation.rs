use std::fs;
use std::path::Path;
use std::process;

pub fn creation(name: &str, lang: &str) {
    println!("Creating {} in {}", name, lang);

    let base_path = Path::new(name);
    let src_path = base_path.join("src");

    if let Err(e) = fs::create_dir_all(&src_path) {
        eprintln!("Cannot create directory {}: {}", src_path.display(), e);
        process::exit(1);
    }

    let toml_path = base_path.join("Jake.toml");
    let content = format!("[project]\nname = \"{}\"\nlang = \"{}\"", name, lang);
    if let Err(e) = fs::write(&toml_path, content) {
        eprintln!("Cannot create file {}: {}", toml_path.display(), e);
        process::exit(1);
    }
}