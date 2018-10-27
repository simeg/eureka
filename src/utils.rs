pub mod utils {
    use std::env;
    use std::fs;

    pub fn is_program_in_path(program: &str) -> bool {
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

    pub fn format_idea_filename(idea: &String) -> String {
        idea.replace(" ", "_").to_uppercase()
    }
}
