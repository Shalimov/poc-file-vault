// use crate::controllers::image_controller;
// use actix_web::{middleware, App, HttpServer};

mod controllers;
mod domain;
// mod utils;

// #[actix_web::main]
fn main() -> std::io::Result<()> {
  Ok(())

  // std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
  // std::fs::create_dir_all("./tmp").unwrap();

  // let ip = "0.0.0.0:8000";

  // HttpServer::new(|| {
  //     App::new()
  //         .wrap(middleware::Logger::default())
  //         .service(image_controller::image_upload)
  // })
  // .bind(ip)?
  // .run()
  // .await
}
