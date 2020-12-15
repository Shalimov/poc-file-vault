use actix_multipart::{Field, Multipart};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use futures::StreamExt;

pub async fn split_payload(payload: &mut Multipart) -> Bytes {
    let mut file_data = BytesMut::with_capacity(10 * 1024 * 1024);

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let content_type = field.content_disposition().unwrap();

        match content_type.get_filename() {
            Some(_filename) => {
                while let Some(chunk) = field.next().await {
                    let data: Bytes = chunk.unwrap();
                    file_data.put(data);
                }
            }
            None => {
                println!("file not provided");
            }
        }
    }
    file_data.to_bytes()
}
