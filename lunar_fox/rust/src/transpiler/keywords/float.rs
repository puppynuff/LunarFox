use crate::transpiler::token_classes::variable_keyword::define;

pub fn float(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "f32".to_string());
}

pub fn double(old_tokens : Vec<String>, name : String, value : String, index : usize, old_index_change : usize, constant : bool) -> (Vec<String>, usize) {
    return define(old_tokens, name, value, index, old_index_change, constant, "f64".to_string());
}