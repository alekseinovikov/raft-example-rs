use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::repository::{NodeInfo, NodeRole, Repository};

use api::api::coordinator_server::Coordinator;
use api::api::{GetAllNodesResponseProto, NodeInfoProto, NodeRoleProto, RegisterNodeResponseProto};
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

    async fn get_all_nodes(&self, _request: Request<()>) -> Result<Response<GetAllNodesResponseProto>, Status> {
        let nodes = self.repository.get_nodes().await;
        let nodes_proto: Vec<NodeInfoProto> = nodes.into_iter().map(|node| node.into()).collect();
        Ok(Response::new(GetAllNodesResponseProto { nodes: nodes_proto }))
    }
}

impl Into<NodeInfo> for NodeInfoProto {
    fn into(self) -> NodeInfo {
        NodeInfo {
            uuid: self.uuid,
            address: self.address,
            role: self.role.into(),
        }
    }
}

impl Into<NodeInfoProto> for NodeInfo {
    fn into(self) -> NodeInfoProto {
        NodeInfoProto {
            uuid: self.uuid,
            address: self.address,
            role: self.role.into(),
        }
    }
}

impl Into<NodeRole> for i32 {
    fn into(self) -> NodeRole {
        let proto = NodeRoleProto::try_from(self).unwrap_or_default();
        match proto {
            NodeRoleProto::NodeRoleFollower => NodeRole::Follower,
            NodeRoleProto::NodeRoleLeader => NodeRole::Leader,
            NodeRoleProto::NodeRoleCandidate => NodeRole::Candidate,
        }
    }
}

impl Into<i32> for NodeRole {
    fn into(self) -> i32 {
        let proto = match self {
            NodeRole::Follower => NodeRoleProto::NodeRoleFollower,
            NodeRole::Leader => NodeRoleProto::NodeRoleLeader,
            NodeRole::Candidate => NodeRoleProto::NodeRoleCandidate,
        };

        proto.into()
    }
}