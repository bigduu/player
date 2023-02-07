#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_video_path() -> Vec<String> {
    let exe = env::current_exe().unwrap();
    let dir = env::current_dir().unwrap();
    let current_dir = dir.to_str().unwrap();
    let current_exe = exe.to_str().unwrap();
    println!("current_dir: {}", current_dir);
    println!("current_exe: {}", current_exe);
    let entries = std::fs::read_dir(".").unwrap();
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        let path = path.to_str().unwrap().to_string();
        println!("path: {}", path);
        if path.ends_with(".mp4") {
            paths.push(path);
        }
    }
    paths.push(current_dir.to_string());
    paths.push(current_exe.to_string());
    paths
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
