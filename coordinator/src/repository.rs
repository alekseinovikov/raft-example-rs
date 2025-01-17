use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;
use common::NodeInfo;

pub(crate) struct Repository {
    nodes: Arc<Mutex<HashMap<String, NodeInfo>>>,
}

impl Repository {
    pub(crate) fn new() -> Self {
        Self {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub(crate) async fn add_node(&self, node: NodeInfo) {
        let mut nodes = self.nodes.lock().await;
        info!("Adding node: {:?}", node);
        nodes.insert(node.uuid.clone(), node);
    }

    pub(crate) async fn get_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.lock().await;
        info!("Getting nodes: {:?}", nodes.values());
        nodes.values().cloned().collect()
    }

    pub (crate) async fn remove_node(&self, uuid: &str) {
        let mut nodes = self.nodes.lock().await;
        info!("Removing node with uuid: {:?}", uuid);
        nodes.remove(uuid);
    }
}
