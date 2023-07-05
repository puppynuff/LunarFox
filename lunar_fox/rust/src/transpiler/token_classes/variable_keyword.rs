pub fn run_token(old_tokens: Vec<String>, tokens : Vec<String>, index : usize, old_index_change : usize, function : fn(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize)) -> (Vec<String>, usize) {
    let mut constant = false;
    let mut new_tokens = old_tokens.clone();
    let mut index_change = old_index_change;

    if new_tokens[index - index_change - 1].trim() == "const".to_string() {
        constant = true;
    }

    let name = &tokens[index + 1];
    let mut value = tokens[index + 3].clone();

    if !value.contains("\"") {
        value = value.replace(";", "");
    }

    let (edited_new_tokens, edited_index_change) = function(new_tokens, name.to_string(), value.to_string(), index, index_change, constant);

    new_tokens = edited_new_tokens.clone();
    index_change = edited_index_change;

    return (new_tokens, index_change);

}

pub fn define(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool, var_type : String) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : {} = {};\n", name, var_type, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : {} = {};\n", name, var_type, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}