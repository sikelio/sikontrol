// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::net::IpAddr;

use local_ip_address::local_ip;

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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, get_package_json, get_package_rust, get_ip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
