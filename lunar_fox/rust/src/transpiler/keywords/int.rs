use crate::transpiler::token_classes::variable_keyword::define;

pub fn int8(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "i8".to_string());
}

pub fn short(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "i16".to_string());
}

pub fn int(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "i32".to_string());
}

pub fn long(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "i64".to_string());
}

pub fn int128(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "i128".to_string());
}

pub fn arch(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "isize".to_string());
}