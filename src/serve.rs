use actix_files::Files;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(Files::new("/media", "/media").show_files_listing()))
        .bind("0.0.0.0:29839")?
        .run()
        .await
}
