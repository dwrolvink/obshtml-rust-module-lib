use crate::stdlib::*;
use crate::lib::errors::{Error};
use crate::module::baseclass::{ObsidianModule, ObsidianModuleConfig};
use crate::cli::config::{Config, RunConfig, AcceptConfig};
use crate::cli::execute;

// EXECUTE
// ==================================================================================

pub fn start(obs_cfg: ObsidianModuleConfig){
    // get commandline arguments, and cast it to a config object
    let config = Config::new();

    // execute the correct method based on type of config object
    let res = match config {
        Config::RunConfig(cli_cfg) => execute::execute_run(cli_cfg, obs_cfg),
        Config::AcceptConfig(cli_cfg) =>  execute::execute_accept(cli_cfg, obs_cfg),
    };
}

pub fn execute_run(cli_cfg: RunConfig, obs_cfg: ObsidianModuleConfig) -> Result<String, Error> {
    let obsmod = ObsidianModule::new(&obs_cfg);
    println!("Running run_fn for:\n\t{}", obsmod.nametag());

    let mdf = cli_cfg.module_data_folder.to_string();
    (obs_cfg.run_fn)(mdf);

    return Ok(format!("done with {}", cli_cfg.command));
}
pub fn execute_accept(cli_cfg: AcceptConfig, obs_cfg: ObsidianModuleConfig) -> Result<String, Error> {
    //dbg!(obs_cfg.run_fn);
    let obsmod = ObsidianModule::new(&obs_cfg);
    println!("Running accept_fn for:\n\t{}", obsmod.nametag());
    
    let mdf = cli_cfg.module_data_folder.to_string();
    (obs_cfg.accept_fn)(mdf);
    
    return Ok(format!("done with {}", cli_cfg.command));
}

