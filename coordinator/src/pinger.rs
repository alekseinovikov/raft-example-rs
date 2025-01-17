use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport;
use tracing::info;
use transport::Channel;
use crate::repository::{NodeInfo, Repository};
use api::api::ping_node_client::PingNodeClient;

pub(crate) struct Pinger {
    repository: Arc<Repository>,
    connections: Mutex<HashMap<String, Arc<Mutex<PingNodeClient<Channel>>>>>
}

impl Pinger {
    pub(crate) fn new(repository: Arc<Repository>) -> Self {
        Self {
            repository,
            connections: Mutex::new(HashMap::new())
        }
    }

    pub(crate) async fn check_all(&self) {
        let all_nodes = self.repository.get_nodes().await;
        for node in all_nodes {
            let ok = self.check_node(&node).await;
            if !ok {
                info!("Node {} is not reachable", node.uuid);
                self.remove_node(&node.uuid).await;
            }
        }
    }

    async fn check_node(&self, node: &NodeInfo) -> bool {
        let address = &node.address;
        let mut connections = self.connections.lock().await;
        let client = connections.get(node.uuid.as_str());
        let client: Arc<Mutex<PingNodeClient<Channel>>> = match client {
            Some(client) => client.clone(),
            None => {
                let client = PingNodeClient::connect(address.clone()).await;
                match client {
                    Ok(client) => {
                        let client = Arc::new(Mutex::new(client));
                        connections.insert(node.uuid.clone(), client.clone());
                        client.clone()
                    }
                    Err(_) => return false
                }
            }
        };

        let mut client = client.lock().await;
        let response = client.ping(()).await;

        info!("Ping response: {:?} from {}", response, node.uuid);

        response.is_ok()
    }

    async fn remove_node(&self, uuid: &str) {
        let mut connections = self.connections.lock().await;
        connections.remove(uuid);
        self.repository.remove_node(uuid).await;
    }
}