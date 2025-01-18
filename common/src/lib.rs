use tokio::sync::broadcast::Sender;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use api::api::{NodeInfoProto, NodeRoleProto};
use config::Config;

pub mod config;

#[derive(Clone, Debug)]
pub enum NodeRole {
    Follower,
    Leader,
    Candidate,
}

#[derive(Clone, Debug)]
pub struct NodeInfo {
    pub uuid: String,
    pub host: String,
    pub port: String,
    pub role: NodeRole,
}

impl NodeInfo {
    pub fn from(config: &Config) -> Self {
        NodeInfo {
            uuid: config.uuid.clone(),
            host: config.host.clone(),
            port: config.port.clone(),
            role: NodeRole::Follower,
        }
    }
}

impl Into<NodeInfo> for NodeInfoProto {
    fn into(self) -> NodeInfo {
        NodeInfo {
            uuid: self.uuid,
            host: self.host,
            port: self.port,
            role: self.role.into(),
        }
    }
}

impl Into<NodeInfoProto> for NodeInfo {
    fn into(self) -> NodeInfoProto {
        NodeInfoProto {
            uuid: self.uuid,
            host: self.host,
            port: self.port,
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

pub fn init_logger() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

pub async fn handle_shutdown_signal(shutdown_sender: Sender<()>) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for shutdown signal");

    info!("Shutdown signal received");
    let _ = shutdown_sender.send(());
}

