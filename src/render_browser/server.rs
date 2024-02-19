use actix_web::{App, HttpServer};
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
pub async fn launch_server() -> std::io::Result<()> {
    HttpServer::new(move || {
        let generated = generate();
        App::new().service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
