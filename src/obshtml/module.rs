use super::verbosity::{verbose_enough, Verbosity, MessageVerbosity, ConfiguredVerbosity};

// DATACLASS
// =============================================================================================
pub struct ObsidianModule {
    pub module_name: String,
    pub module_class_name: String,
    verbosity_overwrite: Option<ConfiguredVerbosity>,
    pub persistent: bool,
    //pub states:
}
impl Default for ObsidianModule {
    fn default() -> ObsidianModule {
        ObsidianModule {
            module_name: "<module_name>".to_string(),
            module_class_name: "ObsidianModule".to_string(),
            verbosity_overwrite: None,
            persistent: false,
        }
    }
}

/*
    // usage of Default:
    let obsmod = ObsidianModule { module_name: "rust_module".to_string(), ..Default::default() };
*/

// struct states {

// }

// INSTANTIATION
// =============================================================================================
// required parameters for instantiating thes ObsidianModule via new()
pub struct ObsidianModuleConfig<'a> {
    pub module_name: &'a str,
    pub module_class_name: &'a str,
}

// recommended way to instantiate a new ObsidianModule struct
pub fn new(config: ObsidianModuleConfig) -> ObsidianModule {
    let obsmod = ObsidianModule{
        module_name: config.module_name.to_string(),
        module_class_name: config.module_class_name.to_string(),
        ..Default::default()
    };
    return obsmod;
}

/*
    let obsmod = obshtml::module::new(ObsidianModuleConfig{
        module_name: "hello",
        module_class_name: "rust_module",
    });
*/

// METHODS
// =============================================================================================
impl ObsidianModule {
    // return f'{self.module_name} ({self.module_class_name})'
    pub fn nametag(&self) -> String {
        return format!("{} ({})", self.module_name, self.module_class_name);
    }
}


// def __init__(self, module_data_folder, module_name, persistent=None):
// # overwrites
// self.__verbosity__overwrite__ = None

// # set values
// self.set_module_data_folder_path(module_data_folder)

// self.module_class_name = self.__class__.__name__
// self.module_name = module_name
// self.test_module_validity()

// self.persistent = persistent
// self.states = {}
// self.states["cancelled_run"] = False

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