use super::verbosity::{verbose_enough, ConfiguredVerbosity, MessageVerbosity};

use super::super::lib::errors;
use super::super::lib::paths::{RelativePosixPath, AbsolutePosixPath, PosixPath};
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
    pub states: ObsidianModuleStates,
    verbosity_overwrite: Option<ConfiguredVerbosity>,
}
impl Default for ObsidianModule {
    fn default() -> ObsidianModule {
        ObsidianModule {
            module_name: "<module_name>".to_string(),
            module_class_name: "ObsidianModule".to_string(),
            verbosity_overwrite: None,
            persistent: false,
            states: ObsidianModuleStates{
                cancelled_run: false
            }
        }
    }
}
/*
    // usage of Default:
    let obsmod = ObsidianModule { module_name: "rust_module".to_string(), ..Default::default() };
*/

pub struct ObsidianModuleStates {
    cancelled_run: bool
}

// pub trait ObsidianModuleTrait {
//     fn new(ObsidianModuleConfig) -> ObsidianModule;

//     fn requires(&self) -> Option<Vec<String>>;
//     fn provides(&self) -> Option<Vec<String>>;
//     fn alters(&self) -> Option<Vec<String>>;
//     fn accept(&self, module_data_folder: Option<AbsolutePosixPath>) -> bool;
//     fn run(&self);

//     fn nametag(&self) -> String;
//     fn verbose_enough(&self, configured_verbosity: ConfiguredVerbosity, message_verbosity: MessageVerbosity,) -> bool 

// }

// INSTANTIATION
// =============================================================================================
// required parameters for instantiating thes ObsidianModule via new()
pub struct ObsidianModuleConfig<'a> {
    pub module_name: &'a str,
    pub module_class_name: &'a str,
    pub persistent: bool,
}

// METHODS
// =============================================================================================
impl ObsidianModule {

    // recommended way to instantiate a new ObsidianModule struct
    pub fn new(config: ObsidianModuleConfig) -> ObsidianModule {
        let obsmod = ObsidianModule {
            module_name: config.module_name.to_string(),
            module_class_name: config.module_class_name.to_string(),
            persistent: config.persistent,
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

    // def accept(self, module_data_folder):
    //     """This function is run before run(), if it returns False, then the module run is skipped entirely. Any other value will be accepted"""
    //     return

    // @abstractmethod
    // def run(self, module_data_folder):
    //     """Single entrypoint that will be called when it is time for the module to do its thing"""
    //     print("I am useless! Overwrite me!")

    // def _integrate_ensure_module_data_folder(self):
    //     """Used to integrate a module with the current flow, to become deprecated when all elements use modular structure"""
    //     Path(self.module_data_folder).mkdir(exist_ok=True)

    // def integrate_load(self, pb):
    //     """Used to integrate a module with the current flow, to become deprecated when all elements use modular structure"""
    //     raise Exception(f"integrate_load not implemented for module class {self.module_class_name}")

    // def integrate_save(self, pb):
    //     """Used to integrate a module with the current flow, to become deprecated when all elements use modular structure"""
    //     raise Exception(f"integrate_save not implemented for module class {self.module_class_name}")


}


// # set values
// self.set_module_data_folder_path(module_data_folder)

// self.test_module_validity()


// # shortcuts
// self.verbose_enough = verbose_enough

// # init
// self._stash = {}  # see self.stash()

// # records
// self.written_files = ResourceAccessLog()
// self.read_files = ResourceAccessLog()
// self.stored_keys = ResourceAccessLog()
// self.retrieved_keys = ResourceAccessLog()

// # module config
// self.mod_config = {}
// self.define_mod_config_defaults()
// if self.module_data_folder is not None:
//     self.try_load_mod_config()


// TESTS
// =============================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn get_obsmod() -> ObsidianModule {
        return new(ObsidianModuleConfig{
            module_name: "hello",
            module_class_name: "<rust impl>",
            persistent: false,
        });
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

/*
    
*/