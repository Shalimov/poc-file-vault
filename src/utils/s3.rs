use bytes::{Buf, Bytes};
use futures::StreamExt;
use rusoto_core::Region;
use rusoto_s3::S3;
use rusoto_s3::{DeleteObjectRequest, PutObjectRequest, S3Client};
use std::io::Read;

pub struct Client {
    #[allow(dead_code)]
    region: Region,
    s3: S3Client,
    bucket_name: String,
}

impl Client {
    // construct S3 testing client
    pub fn new() -> Client {
        let region = Region::default();

        Client {
            region: region.to_owned(),
            s3: S3Client::new(region),
            bucket_name: "file-vault-debug-test-w2cjebtqm333",
        }
    }

    // pub fn generate_bucket_name(&self, key: &str) -> String {
    //     self.bucket_name_template.replace("{ORGANIZATION_UID}", key)
    // }

    pub async fn put_object(&self, file_buffer: Bytes) -> String {
        let mut contents: Vec<u8> = Vec::new();
        let _ = file.read_to_end(&mut contents);

        let put_request = PutObjectRequest {
            bucket: self.bucket_name,
            key: key.to_owned(),
            body: Some(file_buffer.to_),
            ..Default::default()
        };
        let _res = self
            .s3
            .put_object(put_request)
            .await
            .expect("Failed to put test object");

        self.url(key)
    }

    pub async fn delete_object(&self, key: String) {
        let delete_object_req = DeleteObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: key.to_owned(),
            ..Default::default()
        };

        let _res = self
            .s3
            .delete_object(delete_object_req)
            .await
            .expect("Couldn't delete object");
    }
}
