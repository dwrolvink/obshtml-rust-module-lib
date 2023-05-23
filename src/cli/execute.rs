use super::super::stdlib::*;
use super::super::lib::errors::{Error};
use super::super::module::module::{ObsidianModule, ObsidianModuleConfig};
use super::config::{Config, RunConfig, AcceptConfig};

// EXECUTE
// ==================================================================================
pub fn execute_run(cli_cfg: RunConfig, obs_cfg: ObsidianModuleConfig) -> Result<String, Error> {
    let obsmod = ObsidianModule::new(obs_cfg);
    println!("{}", obsmod.nametag());
    return Ok(format!("done with {}", cli_cfg.command));
}
pub fn execute_accept(cli_cfg: AcceptConfig, obs_cfg: ObsidianModuleConfig) -> Result<String, Error> {
    let obsmod = ObsidianModule::new(obs_cfg);
    println!("{}", obsmod.nametag());
    println!("module data folder: {}", cli_cfg.module_data_folder.to_string());
    return Ok(format!("done with {}", cli_cfg.command));
}

