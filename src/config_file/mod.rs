pub mod model;
pub mod handler;

use crate::toolbelt::file_handler::ReadFile;

use std::env;
use std::path::Path;
use dirs;

pub use model::{
    ConfigurationAPI
};
pub use model::aws::{
    AwsKeySecret,
};
pub use handler::{
    ConfigurationHandler
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationControl {
    pub config: ConfigurationAPI,
    pub aws_credentials: AwsKeySecret,
}

impl ConfigurationControl {

    pub fn new() -> ConfigurationControl {
        Default::default()
    }

    pub fn load(mut self) -> ConfigurationControl {
        let config_file = self.load_config("config.yaml");

        self.config = ConfigurationHandler::load_manager_config(&config_file);
        self.set_aws_env();

        self
    }

    pub fn load_config(&mut self, config: &str) -> String {
        let reader = ReadFile::new();
        let mut home = match dirs::home_dir() {
            Some(path) => path,
            None => Path::new(&std::env::var("CONFIG_SIGNEDURL_BASE").expect("~/")).to_path_buf(),
        };
        home.push(".signedurl");
        home.push(config);
        reader.load(
            home
        )
    }

    pub fn set_aws_env(&self) {
        let creds = self.get_credentials();
        match env::var("AWS_ACCESS_KEY_ID") {
            Ok(env_val) => {
                if env_val.is_empty() {
                    env::set_var(
                        "AWS_ACCESS_KEY_ID",
                        creds.key,
                    );
                }
            },
            Err(_) => {
                env::set_var(
                    "AWS_ACCESS_KEY_ID",
                    creds.key,
                );
            },
        }
        match env::var("AWS_SECRET_ACCESS_KEY") {
            Ok(env_val) => {
                if env_val.is_empty() {
                    env::set_var(
                        "AWS_SECRET_ACCESS_KEY",
                        creds.secret,
                    );
                }
            },
            Err(_) => {
                env::set_var(
                    "AWS_SECRET_ACCESS_KEY",
                    creds.secret,
                );
            },
        }
        env::remove_var("AWS_CREDENTIAL_EXPIRATION");
    }

    pub fn get_credentials(&self) -> AwsKeySecret {
        self.clone().config.specs.credentials.aws.unwrap()
    }

}
