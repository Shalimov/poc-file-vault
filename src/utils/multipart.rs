use actix_multipart::Multipart;
use actix_web::Error;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use futures::{StreamExt, TryStreamExt};

pub async fn split_payload(payload: &mut Multipart) -> Result<(Option<String>, Bytes), Error> {
    let mut file_data = BytesMut::with_capacity(10 * 1024 * 1024);
    let mut filename: Option<String> = None;

    while let Some(mut field) = payload.try_next().await? {
        let content_type = field.content_disposition().unwrap();
        filename = Some(content_type.get_name().unwrap().clone().to_owned());

        while let Some(chunk) = field.next().await {
            let data: Bytes = chunk.unwrap();
            file_data.put(data);
        }
    }

    Ok((filename, file_data.to_bytes()))
}
