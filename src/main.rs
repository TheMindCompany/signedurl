#[allow(unused_imports)]
#[macro_use]
extern crate slog;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_yaml;
extern crate reqwest;
extern crate run_script;
extern crate sys_info;
extern crate regex;
extern crate dialoguer;
extern crate console;
extern crate dirs;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_s3;

extern crate structopt;

mod command_control;
mod signed_url;
mod toolbelt;
mod prompt_user;
mod config_file;

use config_file::ConfigurationControl;
use signed_url::SignedUrlRunner;
use toolbelt::logs::RootLog;

use structopt::StructOpt;

#[tokio::main]
async fn main() {
    // This is the collection of settings sent from the request.
    let cli_options = command_control::CmdCtl::from_args();

    // This should be passed to any underlying modules and follow verbose logic rules for CLI.
    let log_config = RootLog::get_logger(
        cli_options.is_verbose()
    );

    ConfigurationControl::new().load();

    match cli_options.commands {
        Some(command_control::Commands::Configurations(_)) => {
            cli_options.run_command_process();
        },
        None => {
            match SignedUrlRunner::run(&cli_options).await {
                Ok(signed_url) => {
                    info!(log_config, "{:#?}", signed_url.as_str());
                },
                Err(err) => {
                    error!(log_config, "{:#?}", err);
                },
            }
        }
    }
}
