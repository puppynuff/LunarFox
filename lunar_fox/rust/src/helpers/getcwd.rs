// 
pub fn getcwd() -> String {
    let cwd = std::env::current_dir().unwrap();
    return cwd.into_os_string().into_string().unwrap();
}