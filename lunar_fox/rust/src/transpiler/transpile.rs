use std::io::Write;
use crate::transpiler::keywords;
use crate::transpiler::token_classes;

pub fn transpile(tokens : Vec<String>, filepath : String) {
    //! This is definitely going to be the largest file.
    
    let mut new_tokens = tokens.clone();

    //? Without index change, it makes it so that editing more than one keyword will ruin the entire script.
    let mut index_change = 0;

    for (index, token) in new_tokens.clone().iter().enumerate() {
        //? Changed from if statements to match for performance.

        match token.trim() {
            "int8" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::int::int8);
                continue;
            },
            "short" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::int::short);
                continue;
            },
            "int" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::int::int);
                continue;
            },
            "long" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::int::long);
                continue;
            },
            "int128" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::int::int128);
                continue;
            },
            "arch" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::int::arch);
                continue;
            },
            "uint8" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::uint::uint8);
                continue;
            },
            "ushort" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::uint::ushort);
                continue;
            },
            "uint" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::uint::uint);
                continue;
            },
            "ulong" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::uint::ulong);
                continue;
            },
            "uint128" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::uint::uint128);
                continue;
            },
            "uarch" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::uint::uarch);
                continue;
            },
            "float" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::float::float);
                continue;
            },
            "double" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::float::double);
                continue;
            },
            "string" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::string::string);
                continue;
            },
            "str" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::string::str);
                continue;
            },
            "char" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::string::char);
                continue;

            },
            "bool" => {
                (new_tokens, index_change) = token_classes::variable_keyword::run_token(new_tokens, tokens.clone(), index, index_change, keywords::bool::bool);
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