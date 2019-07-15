pub mod utils {
    use std::env;
    use std::fs;
    use std::process;

    pub fn get_if_available(program: &str) -> Option<String> {
        if let Ok(path) = env::var("PATH") {
            for p in path.split(":") {
                let p_str = format!("{}/{}", p, program);
                if fs::metadata(p_str).is_ok() {
                    return Some(String::from(program));
                }
            }
        }
        None
    }

    pub fn exit_w_code(code: i32) {
        process::exit(code);
    }
}
