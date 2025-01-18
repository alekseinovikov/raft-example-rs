use api::api::coordinator_client::CoordinatorClient;
use api::api::NodeInfoProto;
use common::config::Config;
use common::NodeInfo;
use tracing::info;

pub(crate) struct Coordinator {
    client: CoordinatorClient<tonic::transport::Channel>,
    node_info: NodeInfo,
}

impl Coordinator {
    pub(crate) async fn new(
        config: &Config,
        node_info: NodeInfo,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let address = format!("http://{}:{}", config.coordinator_host, config.coordinator_port);
        let client = CoordinatorClient::connect(address).await?;
        Ok(Self { client, node_info })
    }

    pub(crate) async fn register_self(&mut self) {
        let proto: NodeInfoProto = self.node_info.clone().into();
        let response = self.client.register_node(proto).await;
        match response {
            Ok(response) => {
                info!(
                    "Registered self into coordinator successfully: {:?}",
                    response
                );
            }
            Err(error) => {
                panic!("Failed to register self into coordinator: {:?}", error);
            }
        }
    }

    pub(crate) async fn remove_self(&mut self) {
        let proto: NodeInfoProto = self.node_info.clone().into();
        let response = self.client.remove_node(proto).await;
        match response {
            Ok(response) => {
                info!(
                    "Removed self from coordinator successfully: {:?}",
                    response
                );
            }
            Err(error) => {
                panic!("Failed to remove self from coordinator: {:?}", error);
            }
        }
    }

    pub(crate) async fn get_all_available_nodes(&mut self) -> Vec<NodeInfo> {
        let response = self.client.get_all_nodes(tonic::Request::new(())).await;
        match response {
            Ok(response) => {
                let nodes = response.into_inner().nodes;
                let nodes: Vec<NodeInfo> = nodes.into_iter().map(|node| node.into()).collect();
                nodes
            }
            Err(error) => {
                panic!("Failed to get all available nodes: {:?}", error);
            }
        }
    }
}
