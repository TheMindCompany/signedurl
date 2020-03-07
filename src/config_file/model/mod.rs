pub mod credentials;
pub mod aws;

pub use credentials::Credentials;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationAPI {
    kind: String,
    version: String,
    pub specs: ConfigurationSpec,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ConfigurationSpec {
    pub credentials: Credentials,
}

impl ConfigurationAPI {

    pub fn new() -> ConfigurationAPI {
        Default::default()
    }
    
    pub fn from_values(credential: Credentials) -> ConfigurationAPI {
        ConfigurationAPI {
            kind: String::from("config"),
            version: String::from("alpha/1.0"),
            specs: ConfigurationSpec::from_values(credential),
        }
    }

}

impl ConfigurationSpec {

    pub fn from_values(credential: Credentials) -> ConfigurationSpec {
        ConfigurationSpec {
            credentials: credential,
        }
    }

}
