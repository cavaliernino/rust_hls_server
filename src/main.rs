use std::path::PathBuf;
use std::sync::{Mutex};
use std::time::Duration;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, HttpRequest};
use actix_web::rt::time;
use actix_web::rt::spawn;
use actix_files as fs;
use fs::NamedFile;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

struct ServerState {
    seq_num: Mutex<u32>,
}

#[get("/")]
async fn playlist(data: web::Data<ServerState>) -> Result<NamedFile> {
    let mut seq_num = data.seq_num.lock().unwrap();
    *seq_num += 1;
    let path: PathBuf = "./assets/stream_player.m3u8".parse().unwrap();
    Ok(NamedFile::open(path)?
        .use_last_modified(true)
        .use_etag(true))
}


async fn segments(_req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = _req.match_info().query("filename").parse().unwrap();
    Ok(NamedFile::open(path)?
        .use_last_modified(true)
        .use_etag(true))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(ServerState {
        seq_num: Mutex::new(0),
    });

    spawn(async move {
        let mut internal = time::interval(Duration::from_secs(10));
        loop {
            internal.tick().await;

            //cambiar el playlist
        }
    });

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(playlist)
            .route("/{filename:.*}", web::get().to(segments))
    }).workers(4)
        .bind_openssl("127.0.0.1:8088", builder)?
        .run()
        .await
}