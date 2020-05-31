extern crate which;

use std::process;

pub fn get_if_available(program: &str) -> Option<String> {
    match which::which(program) {
        Ok(binary_path) => Some(String::from(
            binary_path
                .to_str()
                .expect("Unable to convert PathBuf -> &str"),
        )),
        Err(_) => None,
    }
}

pub fn exit_w_code(code: i32) {
    process::exit(code);
}
