use crate::stdlib::*;

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