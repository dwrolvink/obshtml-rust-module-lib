use std::fs;

pub fn read_file(file_path: &str) -> Option<String> {
    let contents = fs::read_to_string(file_path);
    return match contents {
        Ok(x) => Some(x),
        Err(e) => {
            println!("read_file function failed with error: \"{}\"", e);
            None
        }
    };
}
