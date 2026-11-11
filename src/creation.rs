use std::fs;
use std::path::Path;
use std::process;
use clap::ValueEnum;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Lang {
    Kotlin,
    Java,
}

pub fn creation(name: &str, lang: Lang) {
    println!("Creating {} in {:?}", name, lang);

    let base_path = Path::new(name);
    let src_path = base_path.join("src");

    if let Err(e) = fs::create_dir_all(&src_path) {
        eprintln!("Cannot create directory {}: {}", src_path.display(), e);
        process::exit(1);
    }

    let (ext, boilerplate) = match lang {
        Lang::Java => (
            "java",
            "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello from Jake!\");\n    }\n}\n",
        ),
        Lang::Kotlin => (
            "kt",
            "fun main(args: Array<String>) {\n    println(\"Hello from Jake!\")\n}\n",
        ),
    };

    let toml_path = base_path.join("Jake.toml");
    let content = format!("[project]\nname = \"{}\"\nversion = \"0.0.0\"\nentry = \"Main.{}\"", name, ext);
    if let Err(e) = fs::write(&toml_path, content) {
        eprintln!("Cannot create file {}: {}", toml_path.display(), e);
        process::exit(1);
    }
    let main_file = src_path.join(format!("Main.{}", ext));
    if let Err(e) = fs::write(&main_file, boilerplate) {
        eprintln!("Cannot create file {}: {}", main_file.display(), e);
        process::exit(1);
    }
}