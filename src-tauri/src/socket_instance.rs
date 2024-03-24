use axum::{routing::get, serve, Router};
use enigo::*;
use socketioxide::{extract::SocketRef, layer::SocketIoLayer, SocketIo};
use std::{future::IntoFuture, sync::Arc};
use tokio::{net::TcpListener, sync::Notify};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub struct SocketInstance {
    io: SocketIo,
    layer: SocketIoLayer,
    notify_shutdown: Arc<Notify>,
}

impl SocketInstance {
    pub fn new() -> Self {
        let (layer, io) = SocketIo::new_layer();
        let notify_shutdown = Arc::new(Notify::new());

        SocketInstance {
            io,
            layer,
            notify_shutdown,
        }
    }

    pub async fn start(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.io.ns("/", SocketInstance::handle_events);

        let local_io: SocketIo = self.io.clone();
        let local_layer: SocketIoLayer = self.layer.clone();
        let local_notify = self.notify_shutdown.clone();

        let app = Router::new()
            .route("/", get(|| async { "Sikontrol" }))
            .with_state(local_io)
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive())
                    .layer(local_layer)
            );

        let address = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&address).await?;
        let server = serve(listener, app).into_future();

        tokio::select! {
            _ = server => {},
            _ = local_notify.notified() => {
                println!("Graceful shutdown initiated.");
            },
        }

        println!("Socket IO instance stopped");
        Ok(())
    }

    pub async fn stop(&self) {
        self.notify_shutdown.notify_one();
    }

    fn handle_events(s: SocketRef) {
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
    }
}
