#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{
    env,
    sync::{Arc, Mutex},
};

use actix_cors::Cors;
use actix_files::{NamedFile, Files};
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use tauri::Manager;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, Layer},
    prelude::__tracing_subscriber_SubscriberExt,
};

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

#[tauri::command]
fn exit_fullscreen() {
    let window_list = WINDOW_LIST.lock().unwrap();
    let window = window_list.get(0).unwrap();
    window.set_fullscreen(false);
}

#[tauri::command]
fn enter_fullscreen() {
    let window_list = WINDOW_LIST.lock().unwrap();
    let window = window_list.get(0).unwrap();
    window.set_fullscreen(true);
}

#[tauri::command]
fn switch_fullscreen() {
    let window_list = WINDOW_LIST.lock().unwrap();
    let window = window_list.get(0).unwrap();
    if window.is_fullscreen().unwrap() {
        window.set_fullscreen(false);
    } else {
        window.set_fullscreen(true);
    }
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/play")]
async fn play() -> String {
    let window_list = WINDOW_LIST.lock().unwrap();
    let window = window_list.get(0).unwrap();
    window.emit("play", {}).unwrap();
    "play".to_string()
}

#[get("/pause")]
async fn pause() -> String {
    let window_list = WINDOW_LIST.lock().unwrap();
    let window = window_list.get(0).unwrap();
    window.emit("pause", {}).unwrap();
    "pause".to_string()
}

#[get("/change_video")]
async fn change_video() -> String {
    let window_list = WINDOW_LIST.lock().unwrap();
    let window = window_list.get(0).unwrap();
    window.emit("change_video", {}).unwrap();
    "change_video".to_string()
}

async fn download_file(req: HttpRequest) -> Result<NamedFile> {
    let path = req.match_info().query("filename");
    let path = format!("/Users/bigduu/Desktop/{}", path);
    Ok(NamedFile::open(path)?)
}

lazy_static::lazy_static! {
    static ref WINDOW_LIST: Arc<Mutex<Vec<tauri::Window>>> = Arc::new(Mutex::new(vec![]));
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = init_tracing();
    tokio::task::spawn_blocking(|| {
        let rt = tokio::runtime::Handle::current();
        rt.block_on(async {
            let server = HttpServer::new(|| {
                App::new()
                    .wrap(Cors::permissive())
                    .service(
                        Files::new("/static", "/Users/bigduu/Desktop")
                            .show_files_listing(),
                    )
                    .route("/download/{filename:.*}", web::get().to(download_file))
                    .route("/", web::get().to(index))
                    .service(play)
                    .service(pause)
                    .service(change_video)
            })
            .bind(("0.0.0.0", 8082))
            .expect("Can not bind to port 8082");
            server.run().await.unwrap();
        })
    });
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_video_path,
            exit_fullscreen,
            enter_fullscreen,
            switch_fullscreen
        ])
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            WINDOW_LIST.lock().unwrap().push(window);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

fn init_tracing() -> WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("play_log", "player");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let subscriber = fmt::Subscriber::builder()
        .with_ansi(false)
        .with_max_level(tracing::Level::DEBUG)
        .finish()
        .with(Layer::default().with_writer(non_blocking).with_ansi(false));
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    guard
}
