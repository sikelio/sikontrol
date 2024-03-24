use axum::{routing::get, serve, Router};
use enigo::*;
use socketioxide::{extract::SocketRef, layer::SocketIoLayer, SocketIo};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub struct SocketInstance {
    io: SocketIo,
    layer: SocketIoLayer
}

impl SocketInstance {
    pub fn new() -> Self {
        let (layer, io) = SocketIo::new_layer();

        SocketInstance {
            io,
            layer,
        }
    }

    pub async fn start(&self, port: u16) -> Result<(), Box<dyn std::error::Error>> {
        self.io.ns("/", |s: SocketRef| {
            s.on("play_pause", || {
                println!("Received 'play_pause event'");

                let mut enigo: Enigo = Enigo::new();
                enigo.key_down(Key::MediaPlayPause);
            });

            s.on("prev_track", || {
                println!("Received 'prev_track event'");

                let mut enigo: Enigo = Enigo::new();
                enigo.key_down(Key::MediaPrevTrack);
            });

            s.on("next_track", || {
                println!("Received 'next_track event'");

                let mut enigo: Enigo = Enigo::new();
                enigo.key_down(Key::MediaNextTrack);
            });
        });

        let local_io: SocketIo = self.io.clone();
        let local_layer: SocketIoLayer = self.layer.clone();

        let app: Router = Router::new()
            .route("/", get(|| async { "Sikontrol" }))
            .with_state(local_io)
            .layer(
                ServiceBuilder::new()
                    .layer(CorsLayer::permissive())
                    .layer(local_layer)
            );

        let mut address: String = "0.0.0.0:".to_owned();
        address.push_str(&port.to_string());

        let listener: TcpListener = TcpListener::bind(address).await.unwrap();
        serve(listener, app).await.unwrap();

        println!("Socket IO instance started");

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
