//? Developed by lunar fox

// hello!

/**
 * ! Current functions
 * ? Command line arguments.
 * ? Auto generated base projects
 * ? Tokenization (Mostly)
 */
mod starter_project;
mod helpers;
mod transpiler;

use std::env;

fn main(){
    let mut args : Vec<String> = env::args().collect();

    args.push("".to_string());

    match args[1].as_str() {
        "init" => return starter_project::setup::create_project(),
        "compile" => {
            transpiler::lexer::lexer(false, false, [].to_vec());
            return;
        },
        "build" => {
            transpiler::lexer::lexer(true, false, [].to_vec());
            return;
        },
        "run" => {
            let mut arguments: Vec<String> = [].to_vec();

            for (index, value) in args.iter().enumerate() {
                if index <= 1 || index == args.len() - 1{
                    continue;
                }

                arguments.push(value.trim().to_string());
            }

            transpiler::lexer::lexer(false, true, arguments);
        }
        _ =>  {
            help();
        }
    }
}

fn help() {
    println!("LunarFox help");
    println!("init");
    println!("      Initializes a project, for easy setup. Includes Cargo.toml and .gitignore");
    println!("compile");
    println!("      When run in the project folder, it will compile the source code. (Transpiles)");
    println!("build");
    println!("      When run in the project folder, it will compile the code and build it through cargo. (Transpile -> build through cargo)");
    println!("run + args...");
    println!("  When run in the project folder, it will compile the code and run it through cargo with given args. (Transpile -> run through cargo)");
    println!("REMEMBER : All args are CASE SENSITIVE. Make sure the capitalization is the same too!");
}