use std::net::SocketAddr;
use tokio::sync::broadcast::Sender;
use tonic::transport::Server;
use tracing::info;
use common::config::Config;
use common::{handle_shutdown_signal, init_logger, NodeInfo};
use crate::coordinator::Coordinator;
use crate::ping::PingServerImpl;
use api::api::ping_node_server::{PingNode, PingNodeServer};

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

    Ok(())
}

async fn start_servers_blocking(
    config: &Config,
    ping_server: PingServerImpl,
    shutdown_sender: Sender<()>
) -> Result<(), Box<dyn std::error::Error>>{
    let addr: SocketAddr = config.address.clone().parse().unwrap();
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