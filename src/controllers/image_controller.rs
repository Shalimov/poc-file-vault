use crate::utils::{multipart, s3};
use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use bytes::Bytes;
use image::imageops::FilterType;
use image::{load_from_memory as load_image, DynamicImage};
use rayon::prelude::*;
use std::borrow::BorrowMut;

const RESIZE_MODES: [u16; 9] = [200, 300, 400, 500, 600, 700, 800, 900, 1000];

pub async fn image_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
  let (filename, file_buf): (Option<String>, Bytes) =
    multipart::split_payload(payload.borrow_mut()).await?;
  let s3_client = s3::Client::new();
  let converted_image: DynamicImage = match load_image(&file_buf) {
    Ok(image_result) => image_result,
    Err(message) => panic!(message),
  };

  let resized_images: Vec<Vec<u8>> = RESIZE_MODES
    .into_par_iter()
    .map(|mode| tranform(&converted_image, (*mode, *mode)))
    .collect();

  let filename = filename
    .unwrap_or("file".to_owned())
    .trim_end_matches(".png")
    .to_owned();

  for (i, image) in resized_images.iter().enumerate() {
    let filename = format!("{}_{}.png", filename, i);
    s3_client.put_object(&filename, image.to_owned()).await;
  }

  Ok(HttpResponse::Ok().into())
}

fn tranform(image: &DynamicImage, size: (u16, u16)) -> Vec<u8> {
  let mut img: Vec<u8> = vec![];

  image
    .resize(size.0 as u32, size.1 as u32, FilterType::Lanczos3)
    .write_to(&mut img, image::ImageOutputFormat::Png)
    .expect("Png image cannot be generated");

  img
}
