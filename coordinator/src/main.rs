use crate::server::CoordinatorServerImpl;
use api::api::coordinator_server::CoordinatorServer;
use pinger::Pinger;
use repository::Repository;
use std::sync::Arc;
use std::time::Duration;
use tokio::{task, time};
use tonic::transport::Server;
use tracing_subscriber::FmtSubscriber;
use tracing::{info, Level};

mod pinger;
mod repository;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logger();

    let repository = Arc::new(Repository::new());
    let pinger = Pinger::new(repository.clone());
    let server = CoordinatorServerImpl::new(repository.clone());

    start_pinger(pinger).await;
    start_server_blocking(server).await
}

async fn start_server_blocking(server: CoordinatorServerImpl) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    info!("Coordinator listening on {}", addr);

    Server::builder()
        .add_service(CoordinatorServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}

async fn start_pinger(pinger: Pinger) {
    let ping_duration = Duration::from_secs(5);
    task::spawn(async move {
        let mut interval = time::interval(ping_duration);

        loop {
            interval.tick().await;
            pinger.check_all().await;
        }
    });
}

fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}
