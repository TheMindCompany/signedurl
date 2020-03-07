mod put;
mod get;
mod delete;
use std::result::Result;
use rusoto_credential::{ChainProvider, ProvideAwsCredentials};
use crate::command_control::cmd_model::cmdctl::CmdCtl;
use std::time::{Duration};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SignedUrlRunner {}

impl SignedUrlRunner {
    pub async fn run(request: &CmdCtl) -> Result<String, ()> {
        let verbose = request.verbose;

        let mut credential_provider = ChainProvider::new();
        credential_provider.set_timeout(Duration::from_millis(200));
        let credential = credential_provider
            .credentials().await
            .expect("Delete of presigned url obj failed");

        match request.method.as_str() {
            "GET" => {
                Ok(get::get_object_with_presigned_url(&request, &credential))
            },
            "POST" | "PUT" | "UPDATE" => {
                Ok(put::put_object_with_presigned_url(&request, &credential))
            },
            "DELETE" | "DEL" | "REMOVE" => {
                Ok(get::get_object_with_presigned_url(&request, &credential))
            },
            _ => {
                Err(())
            },
        }
    }
}
