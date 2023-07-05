//? Created by Lunar fox
use crate::{helpers::getcwd::getcwd, starter_project::file_text_handler};
use std::io::Write;

//* Why does rust have to be like this? 
//* Rust hurts my head
//? To tell the truth, I don't know what half of this does :/
//? Wtf does &name_str.trim()[..] do, and why does it work.
//? Why am I doing this as my first big rust project. I've only made one other thing which was a barely working discord bot.

pub fn create_project() {
    let cwd : String = getcwd();

    let mut name_str = String::new();
    println!("What is the name of your project?");

    std::io::stdin().read_line(&mut name_str).unwrap();

    let name : &str = &name_str.trim()[..];

    println!("The projects directory is going to be {}\\{}\\", cwd, name);

    println!("Does this look correct? (Y/n)");

    let mut line2 = String::new();

    std::io::stdin().read_line(&mut line2).unwrap();

    match &line2.trim()[..] {
        "n" => return println!("Sorry about that! Please restart the program to retry the process."),
        _ => println!("Amazing, lets continue!")
    }

    println!("Generating project...");
    let base_path : &str = &(cwd + "\\" + name);

    create_dirs(base_path);
    create_files(base_path, name);
    

    println!("Project created at {}!\nUse {}\\src for .rs files, and {}\\lunar_fox_src for .lfc files!", base_path, base_path, base_path);
}

fn create_dirs(base_path : &str) {
    let _ = std::fs::create_dir(base_path);
    let _ = std::fs::create_dir(format!("{}\\src", base_path));
    let _ = std::fs::create_dir(format!("{}\\lunar_fox_src", base_path));
}

fn create_files(base_path : &str, name : &str) {
    let mut cargo_toml_file = std::fs::File::create(format!("{}\\Cargo.toml", base_path)).expect("Error ecountered when creating Cargo.toml!");
    let _ = cargo_toml_file.write_all(file_text_handler::cargo_toml_contents(name.to_string()).as_bytes()).expect("Failed to write data to Cargo.toml!");

    let mut gitignore_file = std::fs::File::create(format!("{}\\.gitignore", base_path)).expect("Error ecountered when creating .gitignore");
    let _ = gitignore_file.write_all(file_text_handler::gitignore_contents().as_bytes()).expect("Failed to write data to .gitignore!");

    let mut main_file = std::fs::File::create(format!("{}\\lunar_fox_src\\main.lfc", base_path)).expect("Error ecountered when creating main.lfc!");
    let _ = main_file.write_all(file_text_handler::main_file_contents().as_bytes()).expect("Failed to write data to main.lfc!");
}