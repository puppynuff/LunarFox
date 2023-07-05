pub fn string(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;
    if constant {
        new_tokens[index - index_change] = format!("let {} : String = {}", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : String = {}", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn str(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : &str = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : &str = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn char(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : char = {}", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : char = {}", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}