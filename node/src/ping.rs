use tonic::{Request, Response, Status};
use api::api::ping_node_server::PingNode;

pub(crate) struct PingServerImpl {
}

impl PingServerImpl {
    pub(crate) fn new() -> Self {
        Self {
        }
    }
}

#[tonic::async_trait]
impl PingNode for PingServerImpl {
    async fn ping(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
