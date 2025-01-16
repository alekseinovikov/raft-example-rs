use crate::server::CoordinatorServerImpl;
use api::api::coordinator_server::CoordinatorServer;
use pinger::Pinger;
use repository::Repository;
use std::sync::Arc;
use std::time::Duration;
use tokio::{task, time};
use tonic::transport::Server;

mod pinger;
mod repository;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repository = Arc::new(Repository::new());
    let pinger = Pinger::new(repository.clone());
    let server = CoordinatorServerImpl::new(repository.clone());

    start_pinger(pinger).await;
    start_server(server).await
}

async fn start_server(server: CoordinatorServerImpl) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    println!("Coordinator listening on {}", addr);

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
