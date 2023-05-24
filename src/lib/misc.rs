use std::fs;
use crate::stdlib::*;

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

pub fn expect_at_least_n_args(args: &Vec<String>, n: usize, error_text: &str) {
    // number of args expected minus 1 (the command, arg[0])
    if args.len() < n + 1 {
        panic!("{}",
            format_error(
                "Invalid arguments", 
                format!("Expected at least {} argument(s), got {} arguments", n, args.len() - 1).as_str(),
                error_text
            )
        )
    }
}