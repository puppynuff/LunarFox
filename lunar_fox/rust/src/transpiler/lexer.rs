/**
 * ! How token vectors are generated
 * [[(File name), token1, ...], ...]
 */

use crate::transpiler::transpile;

use crate::helpers;
use glob::glob;

pub fn lexer(build : bool, run : bool, args: Vec<String>) {
    //! All this file does is tokenize the files.
    println!("WARNING : Outputs do not include newlines. Try not to edit them.\nOutputs also do not include comments.");

    let cwd = helpers::getcwd::getcwd();
    
    let lf_src_folder_exists = helpers::path_exists::path_exists(format!("{}\\lunar_fox_src\\", cwd).as_str());
    if lf_src_folder_exists == false {
        panic!("lf Source folder doesn't exist in current directory.\nErr : 1");
    }

    for entry in glob(format!("{}/**/*.lfc", str::replace(cwd.as_str(), "\\", "/")).as_str()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file_content = std::fs::read_to_string(&path).expect("Error reading file!");

                let file_vec = handle_file(file_content);

                transpile::transpile(file_vec, String::from(path.to_string_lossy()));
            },
            Err(e) => println!("{:?}", e),
        }
    }
    println!("Finished transpiling!");

    if build {
        let _ = std::process::Command::new("cargo").arg("build").spawn();
    }

    if run {
        println!("{:?}", std::process::Command::new("cargo").arg("run").args(args).spawn());
    }
}

fn handle_file(file_contents : String) -> Vec<String> {
    let full_list = file_contents.split("\"").collect::<Vec<&str>>();

    let mut quote_list : Vec<&str> = [].to_vec();
    let mut token_blocks: Vec<&str> = [].to_vec();

    let mut token_list : Vec<String> = [].to_vec();

    for (i, value) in full_list.iter().enumerate() {

        let check = i % 2;
        if check == 0 {
            token_blocks.push(value.trim());
        } else {
            quote_list.push(value.trim());
        }
    }

    //? Couldn't think of a better name
    let mut quote_spot: usize = 0;

    for token_block in token_blocks {
        let token_lines = token_block.split("\n");

        for line in token_lines {

            let tokens = line.split(" ");

            for token in tokens {
                if token.contains("//") {
                    break;
                }

                //? Fix the size
                if token.trim() != "" {
                    token_list.push(token.trim().to_string());
                }
            }            
        }
        if quote_list.len() > quote_spot {
            token_list.push(format!("\"{}\"", quote_list[quote_spot]).to_string());
            quote_spot += 1;
        }
    }

    token_list
}