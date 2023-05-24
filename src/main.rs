// extern crate json;

mod module {
    pub mod verbosity;
    pub mod baseclass;
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
// use stdlib::*;

// use lib::paths::{PosixPath, AbsolutePosixPath, RelativePosixPath};

// use cli::config::Config as CliConfig;
// use cli::execute;

// use module::baseclass::{ObsidianModuleConfig};

fn main() {
    println!("{}", "I have no feet but I must run");
}

// fn main() {
//     // test
//     let pxp = PosixPath::AbsolutePosixPath(AbsolutePosixPath("/path/to/files.json".to_string()));
//     println!("my path: {}", pxp.to_string());

//     // say hello
//     out!("starting obsidianhtml rust module...");

//     // configure this custom module
//     let obs_cfg = ObsidianModuleConfig {
//         module_name: "hello",
//         module_class_name: "<rust impl>",
//         persistent: false,
//     };

//     // get commandline arguments, and cast it to a config object
//     let args = sys_argsv();
//     let config = CliConfig::new(&args);

//     // execute the correct method based on type of config object
//     let res = match config {
//         CliConfig::RunConfig(cli_cfg) => execute::execute_run(cli_cfg, obs_cfg),
//         CliConfig::AcceptConfig(cli_cfg) =>  execute::execute_accept(cli_cfg, obs_cfg),
//     };

//     // check result of execution
//     let res = res.unwrap();
//     println!("{}", res);
// }

// fn misc() {
//     let files_json_path = "test/files.json";
//     let contents = lib::read_file(&files_json_path).unwrap_or_else(|| panic!("Fetching contents of {} failed, can't continue", &files_json_path));
//     let data = parse_json(&contents);

//     println!("{:#}", data);
//     println!("Value of index.md: {}", data["index.md"]);
// }

// fn parse_json(json_string: &String) -> json::JsonValue {
//     return json::parse(&json_string).unwrap();
// }
