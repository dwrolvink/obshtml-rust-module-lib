mod obshtml { 
    pub mod verbosity;
    pub mod module; 
}
use obshtml::verbosity::{verbose_enough, Verbosity, MessageVerbosity, ConfiguredVerbosity};
use obshtml::module::{ObsidianModuleConfig};

extern crate json;

fn main() {
    let obsmod = obshtml::module::new(
        ObsidianModuleConfig {
            module_name: "hello",
            module_class_name: "<rust impl>",
        }
    );
    println!("{}", obsmod.nametag());

}

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
