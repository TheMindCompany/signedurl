use crate::signed_url::SignedUrlRunner;
use crate::command_control;
use crate::toolbelt::logs::RootLog;
use structopt::StructOpt;

pub struct Cli { }

impl Cli {
    pub async fn run_as_cli() {
        // This is the collection of settings sent from the request.
        let cli_options = command_control::CmdCtl::from_args();

        // This should be passed to any underlying modules and follow verbose logic rules for CLI.
        let log_config = RootLog::get_logger(
            cli_options.is_verbose()
        );

        match cli_options.commands {
            Some(command_control::Commands::Configurations(_)) => {
                cli_options.clone().run_command_process();
                std::process::exit(0);
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
}
