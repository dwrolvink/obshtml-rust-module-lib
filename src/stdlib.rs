// functions/macros to be included everywhere in this project
/*
    mod stdlib;
    use stdlib::*;
*/

use std;

// quick print statement for lazy fingers
//#[macro_export]
macro_rules! out {
    ($val:expr) => {
        println!("{}", $val);
    };
}
pub(crate) use out;

// get all commandline arguments
pub fn sys_argsv() -> Vec<String> {
    return std::env::args().collect();
}

// used to format panic! inline errors
pub fn format_error(etype: &str, descriptor: &str, msg: &str) -> String {
    let output = format!("\n\tError: {}: {}\n\t{}\n", etype, descriptor, msg);
    return output;
}

