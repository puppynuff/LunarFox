use std::io::Write;
use crate::transpiler::keywords;


pub fn transpile(tokens : Vec<String>, filepath : String) {
    //! This is definitely going to be the largest file.
    
    let mut new_tokens = tokens.clone();

    //? Without index change, it makes it so that editing more than one keyword will ruin the entire script.
    let mut index_change = 0;

    for (index, token) in new_tokens.clone().iter().enumerate() {
        //? Changed from if statements to match for performance.

        match token.trim() {
            "int8" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::int::int8(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
        
                continue;
            },
            "short" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::int::short(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "int" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::int::int(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
        
                continue;
            },
            "long" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::int::long(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
        
                continue;
            },
            "int128" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::int::int128(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
        
                continue;
            },
            "arch" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::int::arch(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
        
                continue;
            },
            "uint8" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::uint::uint8(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "ushort" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::uint::ushort(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "uint" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::uint::uint(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "ulong" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::uint::ulong(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "uint128" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::uint::uint128(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "uarch" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::uint::uarch(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "float" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::float::float(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "double" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::float::double(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "string" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::string::string(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "str" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::string::str(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "char" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::string::char(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;

            },
            "bool" => {
                let mut constant = false;
        
                if new_tokens[index - index_change - 1].trim() == "const".to_string() {
                    constant = true;
                }
        
                let name = &tokens[index + 1];
                let value = &tokens[index + 3].replace(";", "");
        
                let (edited_new_tokens, edited_index_change) = keywords::bool::bool(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);
        
                new_tokens = edited_new_tokens.clone();
                index_change = edited_index_change;
                continue;
            },
            "function" => {
                new_tokens[index - index_change] = "fn".to_string();
                continue;
            },
            _ => {
                if (token.contains("}") || token.contains("{") || token.contains(";")) && !token.contains("\"") {
                    new_tokens[index - index_change].push_str("\n");
                }
            }
        }
    } 

    let out_text = new_tokens.join(" ");
    let out_path = filepath.replace("lunar_fox_src", "src").replace(".lfc", ".rs");

    let mut out_file = std::fs::File::create(out_path).expect("Error creating the file!");
    let _ = out_file.write_all(out_text.as_bytes()).expect("Error writing to file!");
}