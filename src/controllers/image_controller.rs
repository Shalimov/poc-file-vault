use super::dto::UploadInput;
use crate::utils::multipart;
use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse};
use std::borrow::BorrowMut;

#[post("/v1/images/upload")]
pub async fn image_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
  let (payload_data, file_buffer) = multipart::split_payload(payload.borrow_mut()).await;

  let upload_info: UploadInput = serde_json::from_slice(&payload_data).unwrap();

  //make key
  // let s3_upload_key = format!("projects/{}/", "posts_id");
  //create tmp file and upload s3 and remove tmp file
  // let upload_files: Vec<UploadFile> = upload_save_file(file_buffer, s3_upload_key).await.unwrap();
  // println!("upload_files={:#?}", upload_files);
  Ok(HttpResponse::Ok().into())
}
