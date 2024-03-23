use axum::{routing::get, serve, Router};
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

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.io.ns("/", |s: SocketRef| {
            s.on("play_pause", || {
                println!("Received 'play_pause event'");
            });

            s.on("prev_track", || {
                println!("Received 'prev_track event'");
            });

            s.on("next_track", || {
                println!("Received 'next_track event'");
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

        let listener: TcpListener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        serve(listener, app).await.unwrap();

        println!("Socket IO instance started");

        Ok(())
    }

    pub async fn stop(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
