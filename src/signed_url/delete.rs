use crate::command_control::cmd_model::cmdctl::CmdCtl;
use rusoto_core::credential::{AwsCredentials};
use rusoto_s3::util::{PreSignedRequest};
use rusoto_s3::{DeleteObjectRequest};

pub async fn delete_object_with_presigned_url(request: &CmdCtl, credentials: &AwsCredentials) -> String {
    let req = DeleteObjectRequest {
        bucket: request.clone().bucket.unwrap(),
        key: request.clone().key.unwrap(),
        ..Default::default()
    };
    req.get_presigned_url(&request.clone().region(), credentials, &Default::default())
}
