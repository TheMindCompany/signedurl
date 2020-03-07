use structopt::StructOpt;
use structopt::clap::AppSettings::*;
use super::Commands;
use rusoto_core::Region;

#[derive(Debug, StructOpt, Default, Clone)]
#[structopt(
    global_settings = &[DisableVersion, DeriveDisplayOrder, VersionlessSubcommands],
    about = "Generate signed url's for remote storage services."
)]
pub struct CmdCtl {
    /// The type of method being requested for signing url.
    ///
    pub method: String,

    /// Bucket target for signature.
    ///
    #[structopt(short = "b", long = "bucket")]
    pub bucket: Option<String>,

    /// Key path target. (ie: filename)
    #[structopt(short = "k", long = "key")]
    pub key: String,

    /// Region target.
    #[structopt(short = "r", long = "region", default_value="us-east-1")]
    pub region: String,

    /// Duration URL is invalid.
    #[structopt(short = "t", long = "timeout")]
    pub timeout: Option<usize>,

    /// Enable verbose logging.
    #[structopt(long = "verbose", short = "v")]
    pub verbose: bool,

    #[structopt(subcommand)]
    pub commands: Option<Commands>,
}

impl CmdCtl {

    pub fn run_command_process(self) -> CmdCtl {
        match &self.commands {
            Some(commands) => {
                commands.process();
                self
            },
            None => self
        }
    }

    pub fn is_verbose(&self) -> bool {
        match self.commands.clone() {
            Some(commands) => commands.is_verbose(),
            None => self.verbose
        }
    }

    pub fn region(&self) -> Region {
        if self.region.is_empty() {
            Region::UsEast2
        } else {
            Region::UsEast1
        }
    }

}
