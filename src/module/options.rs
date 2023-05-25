use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::Yaml;
use yaml_rust::yaml::Hash;

use crate::lib::misc::{read_file};
use crate::stdlib::*;

pub fn get_configured_options(module_name: &str) -> Option<Yaml> {
    let contents = read_file("/home/dorus/git/obsidian-html/output/mod/config.yml");
    //println!("{}", contents.unwrap());

    let docs = YamlLoader::load_from_str(contents.unwrap().as_str()).unwrap();
    let doc = &docs[0];

    // Debug support
    //println!("{:?}", doc["module_list"]["preparation"]);

    let module_configs = &doc["module_config"];
    let modconf = &module_configs[module_name];

    return match modconf {
        Yaml::BadValue => None,
        _ => Some(modconf.clone())
    };
}

// pub fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

pub fn test() {
    let config = YamlLoader::load_from_str("
a:
  a1: old (should be overwritten)
  a2: old (should not be overwritten)
b: old (should be overwritten)
c: old (only in default)
---
a: 
  a1: new
  a2: 
    invalid: these should error
    invalid2: as type is not string for a2!
b: new
d: new (only in custom, should be ignored) 
").unwrap();

    // YamlLoader accepts multiple documents
    // extract first as we have only one
    let doc1 = &config[0];
    let doc2 = &config[1];
    
    // convert doc to a hash
    let mut defaults = doc1.as_hash().unwrap().clone();   
    let custom = doc2;  // don't unwrap yet, we need this functionality;

    // itermut over default
    // if val is hash, recurse
    // else check if key exists in cust -> copy value over


    fn rec(mut default: Hash, custom: &Yaml) -> Hash {// -> Yaml {
        for entry in default.iter_mut() {
            // get some info
            // println!("> {:?}", entry);

            // unpack
            let key = &entry.0;
            let key_str = key.as_str().unwrap();
            let val = &entry.1;

            // skip entry if config does not have the key
            if variant_eq(&custom[key_str], &Yaml::BadValue) {
                continue;
            }

            // recurse and exit this branch if val = hash
            match val {
                Yaml::Hash(inner) => {
                    *entry.1 = Yaml::Hash(rec(inner.clone(), &custom[key_str]));
                    continue;
                },
                _ => (),
            };

            // update value only if types match
            if ! variant_eq(entry.1, &custom[key_str]) {
                eprintln!("Error: value types do not agree for key {}", key_str);
                continue;
            }

            // Update value
            *entry.1 = custom[key_str].clone();
        }
        return default
    }

    defaults = rec(defaults, custom);
    //print_type_of(&defaults);


    // let options = rec(&mut defaults, custom);

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&Yaml::Hash(defaults)).unwrap(); // dump the YAML object to a String
    }
    println!("{}", out_str);



    /*
    > (String("settinga"), String("bla"))
    > (String("settingb"), String("bleh"))
    */
}

pub fn compile_default_options(options_yaml_str: &str) -> Yaml {
    let default_module_config_options_arr = YamlLoader::load_from_str(options_yaml_str).unwrap();
    let obj = &default_module_config_options_arr[0];
    return obj.clone();
}