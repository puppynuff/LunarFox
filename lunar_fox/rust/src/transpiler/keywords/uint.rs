pub fn uint8(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : u8 = {};", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : u8 = {};", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn ushort(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : u16 = {};", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : u16 = {};", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn uint(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : u32 = {};", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : u32 = {};", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn ulong(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : u64 = {};", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : u64 = {};", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn uint128(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : u128 = {};", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : u128 = {};", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn uarch(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    let mut new_tokens : Vec<String> = old_tokens.clone();
    let mut index_change : usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : usize = {};", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] = format!("let mut {} : usize = {};", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}