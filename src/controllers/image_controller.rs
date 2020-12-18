use crate::utils::{multipart, s3};
use actix_multipart::Multipart;
use actix_web::{post, Error, HttpResponse};
use bytes::Bytes;
use image::imageops::FilterType;
use image::{load_from_memory as load_image, DynamicImage};
use rand::{distributions::Alphanumeric, prelude::*};
use rayon::prelude::*;
use std::borrow::BorrowMut;

const RESIZE_MODES: [u16; 9] = [200, 300, 400, 500, 600, 700, 800, 900, 1000];

#[post("/v1/images/upload")]
pub async fn image_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
  let (_, file_buf): (Option<String>, Bytes) =
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

  let gen_file_name = rand::thread_rng()
    .sample_iter(Alphanumeric)
    .take(10)
    .collect::<String>();

  for (i, image) in resized_images.iter().enumerate() {
    let filename = format!("{}_{}.png", gen_file_name, i);
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
