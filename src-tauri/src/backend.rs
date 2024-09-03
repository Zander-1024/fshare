use anyhow::Result;
use axum::{
    body::Body,
    debug_handler,
    extract::{Path, State},
    http::header,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use http::Method;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

type DB = Arc<RwLock<HashMap<Uuid, PathBuf>>>;

pub async fn app(port: u16) {
    let socket_addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Backend is listening on http://{}", socket_addr);

    let db = DB::default();
    let my_local_ip = local_ip().unwrap();
    let local_ip_port = format!("{}:{}", my_local_ip, port);

    let cors = CorsLayer::new()
        .allow_origin(Any) // 允许所有来源
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST]); // 允许特定方法

    let app = Router::new()
        .route("/generate_url", post(generate_url))
        .route("/downloadfile/:uuid", get(get_data))
        .layer(Extension(local_ip_port))
        .layer(cors)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct FilePath {
    path: PathBuf,
}

#[derive(Deserialize, Serialize)]
struct Url {
    url: String,
}
// 请求主体是一个异步流，只能使用一次。因此，您只能有一个提取器来消耗请求正文。
// axum 通过要求此类提取器作为处理程序采用的最后一个参数来强制执行此操作。
//https://docs.rs/axum/latest/axum/extract/index.html#the-order-of-extractors
async fn generate_url(
    State(data): State<DB>,
    Extension(local_ip): Extension<String>,
    Json(file): Json<FilePath>,
) -> Result<Json<Url>, StatusCode> {
    if file.path.exists() {
        let uuid = Uuid::new_v4();
        if let Ok(mut map) = data.write() {
            map.insert(uuid.clone(), file.path.clone());
        }
        let out_url = format!("http://{}/downloadfile/{:?}", local_ip, uuid);

        Ok(Json(Url { url: out_url }))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[debug_handler]
async fn get_data(State(data): State<DB>, Path(uuid): Path<Uuid>) -> impl IntoResponse {
    let file_path = {
        match data.read() {
            Ok(map) => {
                let file = map.get(&uuid).unwrap();
                file.clone()
            }
            Err(_) => return Err((StatusCode::NOT_FOUND, "File not found")),
        }
    };
    let file_name = file_path.file_name().unwrap().to_str().unwrap();

    let headers = [
        (
            header::CONTENT_TYPE,
            "text/plain; charset=utf-8".to_string(),
        ),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_name),
        ),
    ];

    let file = tokio::fs::File::open(file_path).await.unwrap();
    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok((headers, body))
}
