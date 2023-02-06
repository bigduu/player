#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

use std::env;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_video_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
