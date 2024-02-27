use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;

use crate::input::{Game, InitOptions};
use crate::render_browser::broadcast::Broadcaster;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

async fn do_broadcast_task(broadcaster: Arc<Broadcaster>, lines: Box<dyn Iterator<Item = Game>>) {
    for line in lines {
        let msg = serde_json::to_string(&line).unwrap();
        println!("{}\r", &msg);
        broadcaster.broadcast(&msg).await;
    }
}

#[get("/events")]
async fn event_stream(broadcaster: web::Data<Broadcaster>) -> impl Responder {
    broadcaster.new_client().await
}

#[get("/init-options")]
async fn get_init_options(init_options: web::Data<InitOptions>) -> impl Responder {
    HttpResponse::Ok().json(init_options)
}

#[actix_web::main]
pub async fn launch_server(
    lines: Box<dyn Iterator<Item = Game>>,
    init_options: InitOptions,
    port: u16,
) -> std::io::Result<()> {
    let broadcaster = Broadcaster::create();
    let broadcaster_clone = broadcaster.clone();
    let rc_init_options = Arc::new(init_options);

    let server = HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(web::Data::from(Arc::clone(&broadcaster)))
            .app_data(web::Data::from(Arc::clone(&rc_init_options)))
            .service(event_stream)
            .service(get_init_options)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("0.0.0.0", port))?
    .run();

    let server_task = actix_web::rt::spawn(server);

    let broadcast_task = actix_web::rt::spawn(do_broadcast_task(broadcaster_clone, lines));

    let _ = tokio::try_join!(server_task, broadcast_task).expect("Unable to join tasks");

    Ok(())
}
