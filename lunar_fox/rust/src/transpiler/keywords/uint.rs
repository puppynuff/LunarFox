use crate::transpiler::token_classes::variable_keyword::define;

pub fn uint8(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "u8".to_string());
}

pub fn ushort(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "u16".to_string());
}

pub fn uint(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "u32".to_string());
}

pub fn ulong(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "u64".to_string());
}

pub fn uint128(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "u128".to_string());
}

pub fn uarch(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "usize".to_string());
}