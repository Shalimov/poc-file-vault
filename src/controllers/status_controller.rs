use actix_web::{get, Error, HttpResponse};

#[get("/server-status")]
pub async fn server_status() -> Result<HttpResponse, Error> {
  Ok(HttpResponse::Ok().body("server status is ok"))
}
