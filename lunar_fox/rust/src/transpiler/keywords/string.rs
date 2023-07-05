pub fn string(
    old_tokens: Vec<String>,
    name: String,
    value: String,
    index: usize,
    old_index_change: usize,
    constant: bool,
) -> (Vec<String>, usize) {
    let mut new_tokens: Vec<String> = old_tokens.clone();
    let mut index_change: usize = old_index_change;

    if new_tokens[index - index_change + 4].starts_with(".") {
        if constant {
            new_tokens[index - index_change] = format!(
                "let {} : String = {}{};\n",
                name,
                value,
                new_tokens[index - index_change + 4].replace(";", "")
            )
            .to_string();
            new_tokens.remove(index - index_change - 1);
            index_change += 1;
        } else {
            new_tokens[index - index_change] = format!(
                "let mut {} : String = {}{};\n",
                name,
                value,
                new_tokens[index - index_change + 4].replace(";", "")
            )
            .to_string();
            new_tokens.remove(index - index_change + 4);
        }
    } else {
        if constant {
            new_tokens[index - index_change] =
                format!("let {} : String = {};\n", name, value).to_string();
            new_tokens.remove(index - index_change - 1);
            index_change += 1;
        } else {
            new_tokens[index - index_change] =
                format!("let mut {} : String = {};\n", name, value).to_string();
        }
    }

    let mut remove_dot_token = false;

    if new_tokens[index - index_change + 4].starts_with(".") {
        new_tokens.remove(index - index_change + 3);
        remove_dot_token = true;
    }
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    index_change += 3;

    if remove_dot_token {
        index_change += 1;
    }

    return (new_tokens, index_change);
}

pub fn str(
    old_tokens: Vec<String>,
    name: String,
    value: String,
    index: usize,
    old_index_change: usize,
    constant: bool,
) -> (Vec<String>, usize) {
    let mut new_tokens: Vec<String> = old_tokens.clone();
    let mut index_change: usize = old_index_change;

    if constant {
        new_tokens[index - index_change] =
            format!("let {} : &str = {};\n", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] =
            format!("let mut {} : &str = {};\n", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}

pub fn char(
    old_tokens: Vec<String>,
    name: String,
    value: String,
    index: usize,
    old_index_change: usize,
    constant: bool,
) -> (Vec<String>, usize) {
    let mut new_tokens: Vec<String> = old_tokens.clone();
    let mut index_change: usize = old_index_change;

    if constant {
        new_tokens[index - index_change] = format!("let {} : char = {}", name, value).to_string();
        new_tokens.remove(index - index_change - 1);
        index_change += 1;
    } else {
        new_tokens[index - index_change] =
            format!("let mut {} : char = {}", name, value).to_string();
    }

    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);
    new_tokens.remove(index - index_change + 1);

    index_change += 3;

    return (new_tokens, index_change);
}
