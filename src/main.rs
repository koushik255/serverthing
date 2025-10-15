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
    let video_path = Path::new("/home/koushik/video.mp4");

    let file_size = match fs::metadata(video_path) {
        Ok(meta) => meta.len(),
        Err(_) => return (StatusCode::NOT_FOUND, "file not found").into_response(),
    };

    let range_header = req.headers().get("range").and_then(|r| r.to_str().ok());

     let mut headers = HeaderMap::new();
headers.insert(
        "Access-Control-Allow-Origin",
        HeaderValue::from_static("*"),
    );
    headers.insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, OPTIONS"),
    );
    headers.insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("Range"),
    );
    headers.insert("Content-Type", HeaderValue::from_static("video/mp4"));
    headers.insert("Accept-Ranges", HeaderValue::from_static("bytes"));

    if let Some(range) = range_header {
        // Parse Range: bytes=start-end
        let bytes = range.replace("bytes=", "");
        let parts: Vec<&str> = bytes.split('-').collect();
        let start: u64 = parts[0].parse().unwrap_or(0);
        let end: u64 = parts
            .get(1)
            .and_then(|e| e.parse::<u64>().ok())
            .unwrap_or(file_size - 1);

        let chunk_size = end - start + 1;

        // open file async
        let mut file = match File::open(video_path).await {
            Ok(f) => f,
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "cannot open file").into_response(),
        };

        // seek to desired start position
        if let Err(_) = file.seek(SeekFrom::Start(start)).await {
            return (StatusCode::INTERNAL_SERVER_ERROR, "seek failed").into_response();
        }

        // create a limited stream over the desired chunk
        let limited = file.take(chunk_size);
        let stream = ReaderStream::new(limited);
        let body = Body::from_stream(stream);

        let mut res = Response::new(body);
        *res.status_mut() = StatusCode::PARTIAL_CONTENT;
        res.headers_mut().insert(
            "Content-Range",
            format!("bytes {}-{}/{}", start, end, file_size)
                .parse()
                .unwrap(),
        );
        res.headers_mut().insert("Accept-Ranges", "bytes".parse().unwrap());
        res.headers_mut().insert(
            "Content-Length",
            chunk_size.to_string().parse().unwrap(),
        );
        res.headers_mut().insert("Content-Type", "video/mp4".parse().unwrap());
        return res;
    }

   //cors bs done


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
