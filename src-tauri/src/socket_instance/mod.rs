use axum::{routing::get, serve, Router};
use enigo::*;
use serde::{
    Deserialize, Serialize
};
use serde_json::Value;
use socketioxide::{
    extract::{
        Data, SocketRef
    },
    layer::SocketIoLayer, SocketIo
};
use tauri::{AppHandle, Manager};
use std::{
    future::IntoFuture, sync::{
        Arc, Mutex
    }
};
use tokio::{
    net::TcpListener, sync::Notify
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::audio_controller::AudioController;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ChangeAppVolumeEvent {
    pid: u32,
    volume: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct MuteUnmuteMainEvent {
    action: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct MuteUnmuteSessionEvent {
    pid: u32,
    action: String
}

pub struct SocketInstance {
    io: SocketIo,
    layer: SocketIoLayer,
    notify_shutdown: Arc<Notify>,
    pub is_started: Arc<Mutex<bool>>,
    app_handle: Arc<AppHandle>,
}

impl SocketInstance {
    pub fn new(app_handle: AppHandle) -> Self {
        let (layer, io) = SocketIo::new_layer();
        let notify_shutdown: Arc<Notify> = Arc::new(Notify::new());
        let is_started: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));

        SocketInstance {
            io,
            layer,
            notify_shutdown,
            is_started,
            app_handle: Arc::new(app_handle)
        }
    }

    pub async fn start(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        SocketInstance::setup_socket(&self);

        let local_io: SocketIo = self.io.clone();
        let local_layer: SocketIoLayer = self.layer.clone();
        let local_notify: Arc<Notify> = self.notify_shutdown.clone();

        let app: Router = Router::new()
            .route("/", get(|| async { "Sikontrol" }))
            .with_state(local_io)
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive())
                    .layer(local_layer)
            );

        let address: String = format!("0.0.0.0:{}", port);
        let listener: TcpListener = TcpListener::bind(&address).await?;

        {
            let mut started = self.is_started.lock().unwrap();
            *started = true;
        }

        let server = serve(listener, app).into_future();

        tokio::select! {
            _ = server => {},
            _ = local_notify.notified() => {
                println!("Graceful shutdown initiated.");
            },
        }

        println!("Socket IO instance stopped");

        self.app_handle.emit_all("socket_stopped", "").ok();

        Ok(())
    }

    pub async fn stop(&self) {
        {
            let mut started = self.is_started.lock().unwrap();
            *started = false;
        }

        self.notify_shutdown.notify_one();
    }

    fn setup_socket(&self) {
        let local_app_handle: Arc<AppHandle> = self.app_handle.clone();

        self.io.ns("/", move |s: SocketRef| {
            local_app_handle.emit_all("new_client", &s.id).ok();

            s.emit("sessions", &serde_json::json!({ "sessions": AudioController::get_audio_sessions() })).ok();
            s.emit("main_volume_value", &serde_json::json!({ "value": AudioController::get_main_volume_value() })).ok();

            s.on("play_pause", || {
                let mut enigo: Enigo = Enigo::new();
                enigo.key_down(Key::MediaPlayPause);
            });
    
            s.on("prev_track", || {
                let mut enigo: Enigo = Enigo::new();
                enigo.key_down(Key::MediaPrevTrack);
            });
    
            s.on("next_track", || {
                let mut enigo: Enigo = Enigo::new();
                enigo.key_down(Key::MediaNextTrack);
            });
            
            s.on("change_main_volume", |_s: SocketRef, Data::<String>(volume)| {
                AudioController::change_main_volume(volume.parse::<f32>().unwrap());
            });

            s.on("mute_unmute_main_volume", |_s: SocketRef, Data::<Value>(values)| {
                let data: MuteUnmuteMainEvent = serde_json::from_value(values).unwrap();

                AudioController::mute_unmute_main_volume(data.action);
            });
    
            s.on("change_app_volume", |_s: SocketRef, Data::<Value>(values)| {
                let data: ChangeAppVolumeEvent = serde_json::from_value(values).unwrap();
    
                AudioController::change_app_volume(data.pid, data.volume);
            });

            s.on("mute_unmute_app_volume", |_s: SocketRef, Data::<Value>(values)| {
                let data: MuteUnmuteSessionEvent = serde_json::from_value(values).unwrap();

                AudioController::mute_unmute_app_volume(data.pid, data.action);
            });

            s.on_disconnect(move |s: SocketRef| {
                local_app_handle.emit_all("client_leave", &s.id).ok();
            });
        });
    }
}
