//* This is not a neccessary file, but it is meant to keep ./setup.rs length down.

pub fn cargo_toml_contents(name : String)  -> String {
    format!("[package]\nname = \"{}\"\nversion = \"0.0.1\"\nedition = \"2021\"\n[dependencies]\n#https://doc.rust-lang.org/cargo/reference/manifest.html", name.as_str()).to_string()
}

pub fn gitignore_contents() -> String {
    format!("/target\n/src").to_string()
}

pub fn main_file_contents() -> String {
    //! Keep this updated to the current version of the lexer / transpiler
    "// Just a warning if you wanted to edit the rust itself, it will be overwrited on the next compile, and doesn't include comments / newlines.\n// If you notice, most of this is just default rust. Thats the goal, with small changes to make it fit how I like languages more\n// If you wanted to make the entire thing in rust, you can!\n\n//? Main function\nfn main() {\n  //? Creates a i32 integer\n  int hello = 12;\n  \n  //? Prints hello world and the text.\n  println!(\"Hello, world! {}\", hello);\n}\n".to_string()
}