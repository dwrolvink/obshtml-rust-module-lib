pub mod stdlib;

pub mod module {
    pub mod verbosity;
    pub mod baseclass;
    pub mod options;
    pub mod modfile;
}
pub mod cli {
    pub mod config;
    pub mod execute;
}
pub mod lib {
    pub mod errors;
    pub mod paths;
    pub mod misc;
}

pub type ObsidianModuleConfig<'a> = module::baseclass::ObsidianModuleConfig<'a>;
pub type ObsidianModule = module::baseclass::ObsidianModule;