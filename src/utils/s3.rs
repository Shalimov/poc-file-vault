use rusoto_core::Region;
use rusoto_s3::S3;
use rusoto_s3::{PutObjectRequest, S3Client};

pub struct Client {
    #[allow(dead_code)]
    s3: S3Client,
    bucket_name: String,
}

impl Client {
    // construct S3 testing client
    pub fn new() -> Client {
        let s3client = S3Client::new(Region::default());
        //S3Client::new_with(http_client, provider, Region::default());

        Client {
            s3: s3client,
            bucket_name: "file-vault-debug-test-w2cjebtqm333".to_owned(),
        }
    }

    pub async fn put_object(&self, file_name: &str, file_buffer: Vec<u8>) -> String {
        let put_request = PutObjectRequest {
            bucket: self.bucket_name.clone(),
            key: file_name.to_string(),
            body: Some(file_buffer.into()),
            ..Default::default()
        };

        let _res = self
            .s3
            .put_object(put_request)
            .await
            .expect("Failed to put test object");

        String::from("File Loadeed")
    }
}
