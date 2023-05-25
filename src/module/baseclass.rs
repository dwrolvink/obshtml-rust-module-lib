use yaml_rust::Yaml;

use crate::module::verbosity::{verbose_enough, Verbosity, ConfiguredVerbosity, MessageVerbosity};

use crate::lib::errors;
use crate::lib::paths::{RelativePosixPath, AbsolutePosixPath, PosixPath};
/*
    What does a rust module need to do to be called by ObsidianHtml, and work as intended?
    What needs to be in the python wrapper?
    How will we call the rust module?

    obshtml: do you want to run? rust_module.accept(module_data_folder) -> json(bool)

*/

// DATACLASS
// =============================================================================================
pub struct ObsidianModule {
    pub module_name: String,
    pub module_class_name: String,
    pub persistent: bool,
    pub default_options: Yaml,
    pub states: ObsidianModuleStates,
    pub run_fn: fn(ObsidianModule),
    pub accept_fn: fn(ObsidianModule),
    verbosity_overwrite: Option<ConfiguredVerbosity>,
}
impl Default for ObsidianModule {
    fn default() -> ObsidianModule {
        ObsidianModule {
            module_name: "<module_name>".to_string(),
            module_class_name: "ObsidianModule".to_string(),
            verbosity_overwrite: None,
            persistent: false,
            default_options: Yaml::Null,
            run_fn: placeholder_run_fn,
            accept_fn: placeholder_accept_fn,
            states: ObsidianModuleStates{
                cancelled_run: false
            }
        }
    }
}
fn placeholder_run_fn(obsmod: ObsidianModule) {
    panic!("No run_fn function passed in, this should not be possible");
}
fn placeholder_accept_fn(obsmod: ObsidianModule) {
    panic!("No accept_fn function passed in, this should not be possible");
}
/*
    // usage of Default:
    let obsmod = ObsidianModule { module_name: "rust_module".to_string(), ..Default::default() };
*/

pub struct ObsidianModuleStates {
    cancelled_run: bool
}

// INSTANTIATION
// =============================================================================================
// required parameters for instantiating thes ObsidianModule via new()
pub struct ObsidianModuleConfig<'a> {
    pub module_name: &'a str,
    pub module_class_name: &'a str,
    pub persistent: bool,
    pub default_options: Yaml,
    pub run_fn: fn(ObsidianModule),
    pub accept_fn: fn(ObsidianModule),
}

// METHODS
// =============================================================================================
impl ObsidianModule {

    // recommended way to instantiate a new ObsidianModule struct
    pub fn new(config: &ObsidianModuleConfig) -> ObsidianModule {
        let obsmod = ObsidianModule {
            module_name: config.module_name.to_string(),
            module_class_name: config.module_class_name.to_string(),
            persistent: config.persistent,
            default_options: config.default_options.clone(),
            run_fn: config.run_fn,
            accept_fn: config.accept_fn,
            ..Default::default()
        };
        return obsmod;
    }

    // return f'{self.module_name} ({self.module_class_name})'
    pub fn nametag(&self) -> String {
        return format!("{} ({})", self.module_name, self.module_class_name);
    }

    pub fn verbose_enough(&self, configured_verbosity: ConfiguredVerbosity, message_verbosity: MessageVerbosity,) -> bool {
        return verbose_enough(configured_verbosity, message_verbosity);
    }

    pub fn requires(&self) -> Option<Vec<String>> {
        return None;
    }
    pub fn provides(&self) -> Option<Vec<String>> {
        return None;
    }
    pub fn alters(&self) -> Option<Vec<String>> {
        return None;
    }
    pub fn accept(&self, _module_data_folder: Option<AbsolutePosixPath>) -> bool {
        return true;
    }

    pub fn run(&self, _module_data_folder: Option<AbsolutePosixPath>) -> bool {
        return true;
    }
}


// TESTS
// =============================================================================================

#[cfg(test)]
mod tests {
    use crate::module::baseclass::{ObsidianModule, ObsidianModuleConfig};
    use crate::module::verbosity::{ConfiguredVerbosity, MessageVerbosity, Verbosity};

    fn get_obsmod() -> ObsidianModule {
        let obscfg = ObsidianModuleConfig{
            module_name: "hello",
            module_class_name: "<rust impl>",
            persistent: false,
        };
        return ObsidianModule::new(obscfg);
    }

    #[test]
    fn implement_verbose_enough() {
        let obsmod = get_obsmod();
        let print = obsmod.verbose_enough(ConfiguredVerbosity(Verbosity::Info), MessageVerbosity(Verbosity::Info));
        assert_eq!(print, true);
    }

    #[test]
    fn implement_nametag() {
        let obsmod = get_obsmod();
        let tag = obsmod.nametag();
        assert_eq!(tag, "hello (<rust impl>)");
    }
}