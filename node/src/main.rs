use crate::coordinator::Coordinator;
use crate::ping::PingServerImpl;
use api::api::ping_node_server::PingNodeServer;
use common::config::Config;
use common::{handle_shutdown_signal, init_logger, NodeInfo};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::sync::broadcast::Sender;
use tokio::{task, time};
use tokio::task::JoinHandle;
use tonic::transport::Server;
use tracing::info;

mod coordinator;
mod ping;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let config = Config::load();
    let node_info = NodeInfo::from(&config);

    let (shutdown_sender, _) = tokio::sync::broadcast::channel(2);
    tokio::spawn(handle_shutdown_signal(shutdown_sender.clone()));

    let coordinator = Coordinator::new(&config, node_info).await?;
    let ping_server = PingServerImpl::new();

    let coordinator_handler = start_coordination_job(&config, coordinator, shutdown_sender.clone()).await;
    start_servers_blocking(&config, ping_server, shutdown_sender).await?;

    coordinator_handler.await?;
    Ok(())
}

async fn start_servers_blocking(
    config: &Config,
    ping_server: PingServerImpl,
    shutdown_sender: Sender<()>,
) -> Result<(), Box<dyn std::error::Error>> {
    let address_string = format!("{}:{}", config.host, config.port);
    let addr: SocketAddr = address_string.parse().unwrap();
    info!("Node listening on {}", addr);

    let mut shutdown_receiver = shutdown_sender.subscribe();

    Server::builder()
        .add_service(PingNodeServer::new(ping_server))
        .serve_with_shutdown(addr, async {
            shutdown_receiver.recv().await.ok();
            info!("Server received shutdown signal");
        })
        .await?;

    info!("Server gracefully shut down");
    Ok(())
}

async fn start_coordination_job(
    config: &Config,
    mut coordinator: Coordinator,
    shutdown_sender: Sender<()>
) -> JoinHandle<()> {
    let self_register_duration = Duration::from_secs(config.self_register_duration_seconds);
    let mut interval = time::interval(self_register_duration);
    let mut shutdown_receiver = shutdown_sender.subscribe();

    task::spawn(async move {
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    info!("Register self into coordinator");
                    coordinator.register_self().await;
                },
                _ = shutdown_receiver.recv() => {
                    coordinator.remove_self().await;
                    info!("Node received shutdown signal");
                    break;
                },
            }
        }

        info!("Coordinator gracefully shut down");
    })
}
