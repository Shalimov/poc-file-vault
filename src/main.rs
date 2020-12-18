use actix_web::{middleware, App, HttpServer};

mod controllers;
mod domain;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");

  let ip = "0.0.0.0:3000";

  println!("File-Vault-Server started on address: {}", ip);

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .service(controllers::image_controller::image_upload)
      .service(controllers::status_controller::server_status)
  })
  .bind(ip)?
  .run()
  .await
}
