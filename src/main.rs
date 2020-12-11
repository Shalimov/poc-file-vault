use actix_multipart::Multipart;
use actix_web::{middleware, post, App, Error, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use utils::upload::{save_file as upload_save_file, split_payload, UploadFile};

mod utils;

#[derive(Deserialize, Serialize, Debug)]
pub struct InpAdd {
    pub text: String,
    pub number: i32,
}

#[post("/v1/images/upload")]
async fn image_upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let pl = split_payload(payload.borrow_mut()).await;
    println!("bytes={:#?}", pl.0);
    
    let inp_info: InpAdd = serde_json::from_slice(&pl.0).unwrap();
    println!("converter_struct={:#?}", inp_info);
    println!("tmpfiles={:#?}", pl.1);
    
    //make key
    let s3_upload_key = format!("projects/{}/", "posts_id");
    
    //create tmp file and upload s3 and remove tmp file
    let upload_files: Vec<UploadFile> = upload_save_file(pl.1, s3_upload_key).await.unwrap();
    println!("upload_files={:#?}", upload_files);
    
    Ok(HttpResponse::Ok().into())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    std::fs::create_dir_all("./tmp").unwrap();

    let ip = "0.0.0.0:8000";

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(image_upload)
    })
    .bind(ip)?
    .run()
    .await
}
