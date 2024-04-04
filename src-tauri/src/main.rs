// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod audio_controller;
mod audio_events;
mod socket_instance;

use std::{net::IpAddr, sync::Arc};
use audio_controller::{AudioController, Session};
use local_ip_address::local_ip;
use socket_instance::SocketInstance;
use tauri::{App, AppHandle, Manager, Window};
use tauri_plugin_autostart::MacosLauncher;

const CARGO_TOML: &str = include_str!("../Cargo.toml");
const PACKAGE_JSON: &str = include_str!("../../package.json");

#[tauri::command]
fn get_package_json() -> String {
    PACKAGE_JSON.to_string()
}

#[tauri::command]
fn get_package_rust() -> String {
    CARGO_TOML.to_string()
}

#[tauri::command]
fn get_ip() -> Option<IpAddr> {
    match local_ip() {
        Ok(ip) => Some(ip),
        Err(_) => None
    }
}

#[tauri::command]
async fn start_server(socket_instance: tauri::State<'_, Arc<SocketInstance>>, port: u16) -> Result<(), String> {
    match local_ip() {
        Ok(_ip) => {
            socket_instance.start(port).await.map_err(|e| e.to_string())
        },
        Err(_) => Err("No IP address".to_string())
    }
}

#[tauri::command]
async fn stop_server(socket_instance: tauri::State<'_, Arc<SocketInstance>>) -> Result<String, String> {
    socket_instance.stop().await;
    Ok("Socket instance stopped successfully".to_string())
}

#[tauri::command]
fn is_socket_started(socket_instance: tauri::State<'_, Arc<SocketInstance>>) -> bool {
    let is_started = socket_instance.is_started.lock().unwrap();
    *is_started
}

#[tauri::command]
fn get_sessions() -> Vec<Session> {
    AudioController::get_audio_sessions()
}

#[tauri::command]
async fn close_splashscreen(window: Window) {
    window.get_window("splashscreen").expect("no window labeled 'splashscreen' found").close().unwrap();
    window.get_window("main").expect("no window labeled 'main' found").show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app: &mut App| {
            let app_handle: AppHandle = app.app_handle();
            let socket_instance: Arc<SocketInstance> = Arc::new(SocketInstance::new(app_handle));

            app.manage(socket_instance);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_package_json, get_package_rust, get_ip, start_server,
            stop_server, is_socket_started, get_sessions, close_splashscreen
        ])
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![])))
        .plugin(tauri_plugin_store::Builder::default().build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
