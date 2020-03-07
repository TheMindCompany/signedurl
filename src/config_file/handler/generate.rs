use crate::config_file::ConfigurationAPI;
use crate::config_file::model::aws::{AwsKeySecret};
use crate::toolbelt::file_handler::CreateFile;

use crate::config_file::model::{
    Credentials
};
use std::path::Path;

use dialoguer::theme::ColorfulTheme;
use dialoguer::{Input};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Prompt {
    pub name: String,
    pub default: Option<String>,
}

impl Prompt {
    pub fn new() -> Prompt {
        Default::default()
    }

    pub fn set_name(mut self, name: String) -> Prompt {
        self.name = name;
        self
    }

    pub fn set_default(mut self, default: Option<String>) -> Prompt {
        self.default = default;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct GenerateHandler {
    pub config: ConfigurationAPI,
}

impl GenerateHandler {

    pub fn new() -> GenerateHandler {
        Default::default()
    }

    pub fn run_config_prompt(&mut self) -> GenerateHandler {
        let config = self.get_config();
        let mut config_path = match dirs::home_dir() {
            Some(path) => path,
            None => Path::new(&std::env::var("CONFIG_SIGNEDURL_BASE").expect("~/")).to_path_buf(),
        };
        config_path.push(".signedurl");
        match std::fs::create_dir(&config_path) {
            Ok(_) => {
                println!("Created path ~/.signedurl");
            },
            Err(_) => {
                println!("Path ~/.signedurl already exist.");
            }
        };
        config_path.push("config.yaml");

        CreateFile::new().create(config_path, &serde_yaml::to_string(&config).unwrap());

        self.clone()
    }

    pub fn get_user_credentials(&mut self) -> Credentials {
        // Aws credentials.
        let aws_key = PromptHandler::process( Prompt::new().set_name("aws_key".to_string()) );
        let aws_secret = PromptHandler::process( Prompt::new().set_name("aws_secret".to_string()) );

        Credentials::from_values(Some(AwsKeySecret::new(aws_key, aws_secret)))
    }

    pub fn get_config(&mut self) -> ConfigurationAPI {
        let credential_body = self.get_user_credentials();
        ConfigurationAPI::from_values(credential_body)
    }

}

// Move this to shared module when we know the variance between prompt requirments.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PromptHandler {}

impl PromptHandler {
    pub fn process(prompt: Prompt) -> String {
        PromptHandler::ask_input(
            prompt.default,
            &prompt.name,
        )
    }


    // Prompt for user defined input.
    pub fn ask_input(
        default_value: Option<String>,
        name: &str,
    ) -> String {
        let question = format!("{} should be set?", &name);
        let color_theme = ColorfulTheme::default();
        let mut prompt = Input::with_theme(&color_theme);

        if let Some(x) = default_value {prompt.default(x);}

        prompt
            .with_prompt(&question.as_str())
            .interact()
            .unwrap()
    }
}
