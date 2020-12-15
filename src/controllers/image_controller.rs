use super::dto::UploadInput;
use crate::utils::multipart;
use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse};
use std::borrow::BorrowMut;
use bytes::Bytes;
use image::load_from_memory as load_image;

#[post("/v1/images/upload")]
pub async fn image_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
  let (_payload_data, file_buffer) = multipart::split_payload(payload.borrow_mut()).await;
  
  // tranform(file_buffer, upload_info)


  //make key
  // let s3_upload_key = format!("projects/{}/", "posts_id");
  //create tmp file and upload s3 and remove tmp file
  // let upload_files: Vec<UploadFile> = upload_save_file(file_buffer, s3_upload_key).await.unwrap();
  // println!("upload_files={:#?}", upload_files);
  Ok(HttpResponse::Ok().into())
}



async fn tranform(file_buffer: &Bytes, size: (u16, u16), ) -> Result<(), ()> {
    // let file_vect: Vec<u8> = file_buffer.to_vec();
    
    let converted_image = match load_image(&file_buffer) {
      Ok(image_result) => image_result,
      Err(message) => panic!(message)
    };
    
    converted_image.res

    Ok(())
}
