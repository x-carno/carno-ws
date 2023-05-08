// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, client_list, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn client_list() -> Vec<String> {
    let mut cl = Vec::new();
    cl.push(String::from("localhost1"));
    cl.push(String::from("localhost2"));
    cl.push(String::from("localhost3"));
    cl.push(String::from("localhost4"));
    cl.push(String::from("localhost5"));
    cl

    // vec![String::from("localhost")]
}

#[tauri::command]
fn send_message(msg: &str) {
    println!("send msg: {}", msg);
}
