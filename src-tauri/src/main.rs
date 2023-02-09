#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::env;

use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, Result, web};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_video_path() -> Vec<String> {
    let mut paths = vec![];
    std::fs::read_dir("/Users/bigduu/Desktop")
        .unwrap()
        .for_each(|entry| {
            let path = entry.unwrap().path();
            let path = path.to_str().unwrap().to_string();
            paths.push(path);
        });
    paths
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn download_file(req: HttpRequest) -> Result<NamedFile> {
    let path = req.match_info().query("filename");
    let path = format!("/Users/bigduu/Desktop/{}", path);
    Ok(NamedFile::open(path)?)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::task::spawn_blocking(|| {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let server = HttpServer::new(|| {
                App::new()
                    .service(
                        actix_files::Files::new("/static", "/Users/bigduu/Desktop")
                            .show_files_listing(),
                    )
                    .route("/download/{filename:.*}", web::get().to(download_file))
                    .route("/", web::get().to(index))
            })
                .bind(("127.0.0.1", 8081))
                .expect("Can not bind to port 8081");
            server.run().await.unwrap();
        })
    });
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_video_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}
