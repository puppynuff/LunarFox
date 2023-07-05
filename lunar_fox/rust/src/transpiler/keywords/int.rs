pub fn int8(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : i8 = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : i8 = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn short(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : i16 = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : i16 = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn int(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : i32 = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : i32 = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn long(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : i64 = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : i64 = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn int128(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : i128 = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : i128 = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn arch(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : isize = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : isize = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}