// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod socket_instance;

use std::{error::Error, net::IpAddr};
use local_ip_address::local_ip;
use socket_instance::SocketInstance;
use tauri_plugin_autostart::MacosLauncher;

const CARGO_TOML: &str = include_str!("../Cargo.toml");
const PACKAGE_JSON: &str = include_str!("../../package.json");

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_package_json() -> String {
    PACKAGE_JSON.to_string()
}

#[tauri::command]
fn get_package_rust() -> String {
    CARGO_TOML.to_string()
}

#[tauri::command]
fn get_ip() -> IpAddr {
    let my_local_ip: IpAddr = local_ip().unwrap();

    my_local_ip
}

#[tauri::command]
async fn start_server(socket_instance: tauri::State<'_, SocketInstance>) -> Result<(), String> {
    socket_instance.start().await.map_err(|e: Box<dyn Error>| e.to_string())
}

#[tauri::command]
async fn stop_server(socket_instance: tauri::State<'_, SocketInstance>) -> Result<(), String> {
    socket_instance.stop().await.map_err(|e: Box<dyn Error>| e.to_string())
}

fn main() {
    tauri::Builder::default()
        .manage(SocketInstance::new())
        .invoke_handler(tauri::generate_handler![greet, get_package_json, get_package_rust, get_ip, start_server, stop_server])
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])))
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
