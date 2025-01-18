use std::net::SocketAddr;
use crate::server::CoordinatorServerImpl;
use api::api::coordinator_server::CoordinatorServer;
use pinger::Pinger;
use repository::Repository;
use std::sync::Arc;
use std::time::Duration;
use broadcast::Sender;
use tokio::sync::broadcast;
use tokio::task::JoinHandle;
use tokio::{task, time};
use tonic::transport::Server;
use tracing::info;
use common::config::Config;
use common::{handle_shutdown_signal, init_logger};

mod pinger;
mod repository;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let config = Config::load();

    let repository = Arc::new(Repository::new());
    let pinger = Pinger::new(repository.clone());
    let server = CoordinatorServerImpl::new(repository.clone());

    let (shutdown_sender, _) = broadcast::channel(2);
    tokio::spawn(handle_shutdown_signal(shutdown_sender.clone()));

    let pinger_handle = start_pinger(&config, pinger, shutdown_sender.clone());
    start_server_blocking(&config, server, shutdown_sender.clone()).await?;

    pinger_handle.await?;
    Ok(())
}

async fn start_server_blocking(
    config: &Config,
    server: CoordinatorServerImpl,
    shutdown_sender: Sender<()>
) -> Result<(), Box<dyn std::error::Error>> {
    let address_string = format!("{}:{}", config.host, config.coordinator_port);
    let addr: SocketAddr = address_string.parse().unwrap();
    info!("Coordinator listening on {}", addr);

    let mut shutdown_receiver = shutdown_sender.subscribe();

    Server::builder()
        .add_service(CoordinatorServer::new(server))
        .serve_with_shutdown(addr, async {
            shutdown_receiver.recv().await.ok();
            info!("Server received shutdown signal");
        })
        .await?;

    info!("Server gracefully shut down");
    Ok(())
}

fn start_pinger(config: &Config, pinger: Pinger, shutdown_sender: Sender<()>) -> JoinHandle<()> {
    let ping_duration = Duration::from_secs(config.ping_duration_seconds);
    let mut interval = time::interval(ping_duration);
    let mut shutdown_receiver = shutdown_sender.subscribe();

    task::spawn(async move {
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    info!("Ping all available nodes");
                    pinger.check_all().await;
                },
                _ = shutdown_receiver.recv() => {
                    info!("Pinger received shutdown signal");
                    break;
                },
            }
        }

        info!("Pinger gracefully shut down");
    })
}
