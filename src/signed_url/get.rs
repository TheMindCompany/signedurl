use crate::command_control::cmd_model::cmdctl::CmdCtl;
use rusoto_core::credential::{AwsCredentials};
use rusoto_s3::util::{PreSignedRequest, PreSignedRequestOption};
use rusoto_s3::{GetObjectRequest};
use std::time::{Duration};

pub async fn get_object_with_presigned_url(request: &CmdCtl, credentials: &AwsCredentials) -> String {
    let mut options = PreSignedRequestOption::default();
    options.expires_in = Duration::from_millis(request.timeout);

    let req = GetObjectRequest {
        bucket: request.clone().bucket.unwrap(),
        key: request.clone().key.unwrap(),
        ..Default::default()
    };
    req.get_presigned_url(&request.clone().region(), credentials, &options)
}
