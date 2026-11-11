use std::fs;
use std::path::Path;
use clap::ValueEnum;
use std::io::Write;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Lang {
    Kotlin,
    Java,
}

pub fn creation(name: &str, lang: Lang, mixed: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("Creating {} in {:?}{}", name, lang, if mixed {" (mixed)"} else {""});
    let base_path = Path::new(name);
    let src_path = base_path.join("src");
    let toml_path = base_path.join("Jake.toml");
    fs::write(&toml_path, "[package]\n")?;
    let mut tomlfile = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&toml_path)?;
    writeln!(tomlfile, "name = \"{}\"", name)?;
    writeln!(tomlfile, "version = \"{}\"", "0.0.0")?;
    if mixed {
    } else {

    }
    Ok(())
}