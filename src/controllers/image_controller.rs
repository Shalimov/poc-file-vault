use crate::utils::{multipart, s3};
use std::fs;
use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse};
use bytes::Bytes;
use image::load_from_memory as load_image;
use image::DynamicImage;
use std::borrow::BorrowMut;

const RESIZE_MODES: [u16; 9] = [200, 300, 400, 500, 600, 700, 800, 900, 1000];

#[post("/v1/images/upload")]
pub async fn image_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
  let file_buf = multipart::split_payload(payload.borrow_mut()).await;
  let s3_client = s3::Client::new();

  for mode in RESIZE_MODES.iter() {
    let image = tranform(&file_buf, (*mode, *mode)).await;
    let path = format!("./rust_content/{}", mode);
    fs::write(path, image);
    // s3_client.put_object(image).await;
  }

  Ok(HttpResponse::Ok().into())
}

async fn tranform(file_buffer: &Bytes, size: (u16, u16)) -> Vec<u8> {
  let mut img: Vec<u8> = vec![];

  let converted_image: DynamicImage = match load_image(&file_buffer) {
    Ok(image_result) => image_result,
    Err(message) => panic!(message),
  };

  converted_image.write_to(&mut img, image::ImageOutputFormat::Png);

  img
}
