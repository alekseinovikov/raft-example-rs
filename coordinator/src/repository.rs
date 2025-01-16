use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub(crate) struct NodeInfo {
    pub(crate) uuid: String,
    pub(crate) host: String,
    pub(crate) port: u32,
}

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
        nodes.insert(node.uuid.clone(), node);
    }

    pub(crate) async fn get_nodes(&self) -> Vec<NodeInfo> {
        let nodes = self.nodes.lock().await;
        nodes.values().cloned().collect()
    }

    pub (crate) async fn remove_node(&self, uuid: &str) {
        let mut nodes = self.nodes.lock().await;
        nodes.remove(uuid);
    }
}
