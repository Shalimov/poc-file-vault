use crate::domain::preset::Preset;
// use crate::utils::upload::{save_file as upload_save_file, split_payload, UploadFile};
// use actix_multipart::Multipart;
// use actix_web::{post, Error, HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::borrow::BorrowMut;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UploadInput {
  pub organization_uid: String,
  pub space_name: String,
  pub preset_name: Option<String>,
  pub preset: Option<Preset>,
  pub tags: Option<HashMap<String, String>>,
}

// #[post("/v1/images/upload")]
// pub async fn image_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
//   let pl = split_payload(payload.borrow_mut()).await;

//   let inp_info: UploadInput = serde_json::from_slice(&pl.0).unwrap();

//   //make key
//   let s3_upload_key = format!("projects/{}/", "posts_id");
//   //create tmp file and upload s3 and remove tmp file
//   let upload_files: Vec<UploadFile> = upload_save_file(pl.1, s3_upload_key).await.unwrap();
//   println!("upload_files={:#?}", upload_files);
//   Ok(HttpResponse::Ok().into())
// }
