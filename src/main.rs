// extern crate json;
extern crate yaml_rust;

mod module {
    pub mod verbosity;
    pub mod baseclass;
    pub mod options;
}
mod cli {
    pub mod config;
    pub mod execute;
}
mod lib {
    pub mod errors;
    pub mod paths;
    pub mod misc;
}

mod stdlib;

use crate::module::options::{test};



fn main() {
    // println!("{}", "I have no feet but I must run");

    test();
}
