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
        if self.region == "us-east-1" {
            Region::UsEast1
        } else if self.region == "us-east-2" {
            Region::UsEast2
        } else if self.region == "ap-east-1" {
            Region::ApEast1
        } else if self.region == "ap-south-1" {
            Region::ApSouth1
        } else if self.region == "ap-northeast-1" {
            Region::ApNortheast1
        } else if self.region == "ap-northeast-2" {
            Region::ApNortheast2
        } else if self.region == "ap-northeast-3" {
            Region::ApNortheast3
        } else if self.region == "ap-southeast-1" {
            Region::ApSoutheast1
        } else if self.region == "ap-southeast-2" {
            Region::ApSoutheast2
        } else if self.region == "ca-central-1" {
            Region::CaCentral1
        } else if self.region == "cn-north-1" {
            Region::CnNorth1
        } else if self.region == "cn-northeast-1" {
            Region::CnNorthwest1
        } else if self.region == "eu-central-1" {
            Region::EuCentral1
        } else if self.region == "eu-north-1" {
            Region::EuNorth1
        } else if self.region == "eu-west-1" {
            Region::EuWest1
        } else if self.region == "eu-west-2" {
            Region::EuWest2
        } else if self.region == "eu-west-3" {
            Region::EuWest3
        } else if self.region == "me-south-1" {
            Region::MeSouth1
        } else if self.region == "sa-east-1" {
            Region::SaEast1
        } else {
            Region::UsEast1
        }
    }

}
