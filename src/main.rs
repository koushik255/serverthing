use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::{io::AsyncReadExt, net::TcpListener};
use tower_http::cors::{CorsLayer,Any};

use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode, HeaderMap, HeaderValue},
    response::IntoResponse,
    
};
use std::{fs, io::SeekFrom,  path::Path};
use tokio::fs::File;
use tokio::io::AsyncSeekExt;
use tokio_util::io::ReaderStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
     let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/video", get(send_video))
        .layer(cors);


    // write address like this to not make typos
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = TcpListener::bind(addr).await?;


    println!("Starig server on {}",addr);
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn hello_world() -> &'static str {
    println!("hti");
    "Hello, world!"
}


async fn send_video(req: Request) -> impl IntoResponse {
    // let video_path = Path::new("/mnt/d/1KOUSHIK/goated edits/yodude.mp4");
    //
    println!("HIT HIT I REPEAT VIDEO HIT");
    println!("i guess you dditn need any of that hugh");
    let video_path = Path::new("/home/koushik/video.mp4");

    let file_size = match fs::metadata(video_path) {
        Ok(meta) => meta.len(),
        Err(_) => return (StatusCode::NOT_FOUND, "file not found").into_response(),
    };

 

    let file = match File::open(video_path).await {
        Ok(f) => f,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "cannot open file").into_response(),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "video/mp4")
        .header("Content-Length", file_size)
        .header("Accept-Ranges", "bytes")
        .body(body)
        .unwrap()
}
