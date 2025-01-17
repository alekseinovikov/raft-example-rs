use std::net::SocketAddr;
use tracing::info;
use api::api::coordinator_client::CoordinatorClient;
use api::api::NodeInfoProto;
use common::config::Config;
use common::NodeInfo;

pub(crate) struct Coordinator {
    client: CoordinatorClient<tonic::transport::Channel>,
    node_info: NodeInfo
}

impl Coordinator {
    pub(crate) async fn new(config: &Config, node_info: NodeInfo) -> Result<Self, Box<dyn std::error::Error>> {
        let client
            = CoordinatorClient::connect(config.coordinator_address.clone()).await?;
        Ok(Self { client, node_info })
    }

    pub (crate) async fn register_self(mut self) {
        let proto: NodeInfoProto = self.node_info.into();
        let response
            = self.client.register_node(proto).await;
        match response {
            Ok(response) => {
                info!("Registered self into coordinator successfully: {:?}", response);
            }
            Err(error) => {
                panic!("Failed to register self into coordinator: {:?}", error);
            }
        }
    }

}
