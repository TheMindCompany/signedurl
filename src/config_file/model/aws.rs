#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AwsKeySecret {
    pub key: String,
    pub secret: String
}

impl AwsKeySecret {
    pub fn new(key: String, secret: String) -> AwsKeySecret {
        AwsKeySecret {
            key,
            secret
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct SecretAndKey {
    pub aws_secret_access_key: Option<String>,
    pub aws_access_key_id: Option<String>,
}

impl SecretAndKey {
    pub fn new(secret: &str, key: &str) -> SecretAndKey {
        SecretAndKey {
            aws_secret_access_key: Some(secret.to_string()),
            aws_access_key_id: Some(key.to_string()),
        }
    }
}
