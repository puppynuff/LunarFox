pub fn bool(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : bool = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : bool = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}