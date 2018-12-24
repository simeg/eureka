pub mod utils {
    use std::env;
    use std::fs;
    use std::process;

    pub fn is_available(program: &str) -> bool {
        if let Ok(path) = env::var("PATH") {
            for p in path.split(":") {
                let p_str = format!("{}/{}", p, program);
                if fs::metadata(p_str).is_ok() {
                    return true;
                }
            }
        }
        false
    }

    pub fn exit_w_code(code: i32) {
        process::exit(code);
    }
}
