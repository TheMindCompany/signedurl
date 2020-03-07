use crate::config_file::model::aws::AwsKeySecret;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Credentials {
    pub aws: Option<AwsKeySecret>,
}

impl Credentials {

    pub fn from_values(aws: Option<AwsKeySecret>) -> Credentials {
        Credentials {
            aws,
        }
    }

}
