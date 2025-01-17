use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::repository::Repository;

use api::api::coordinator_server::Coordinator;
use api::api::{GetAllNodesResponseProto, NodeInfoProto, RegisterNodeResponseProto};
use api::api::register_node_response_proto::RegisterNodeStatus;

pub(crate) struct CoordinatorServerImpl {
    repository: Arc<Repository>,
}

impl CoordinatorServerImpl {
    pub(crate) fn new(repository: Arc<Repository>) -> Self {
        Self {
            repository,
        }
    }
}

#[tonic::async_trait]
impl Coordinator for CoordinatorServerImpl {
    async fn register_node(&self, request: Request<NodeInfoProto>) -> Result<Response<RegisterNodeResponseProto>, Status> {
        let node_info = request.into_inner();
        self.repository.add_node(node_info.into()).await;
        Ok(Response::new(RegisterNodeResponseProto{ status: RegisterNodeStatus::Success.into() }))
    }

    async fn remove_node(&self, request: Request<NodeInfoProto>) -> Result<Response<()>, Status> {
        let node_info = request.into_inner();
        self.repository.remove_node(&node_info.uuid).await;
        Ok(Response::new(()))
    }

    async fn get_all_nodes(&self, _request: Request<()>) -> Result<Response<GetAllNodesResponseProto>, Status> {
        let nodes = self.repository.get_nodes().await;
        let nodes_proto: Vec<NodeInfoProto> = nodes.into_iter().map(|node| node.into()).collect();
        Ok(Response::new(GetAllNodesResponseProto { nodes: nodes_proto }))
    }
}
