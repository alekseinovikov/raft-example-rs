use api::api::node_server::{Node, NodeServer};
use api::api::{GetNodeInfoRequest, GetNodeInfoResponse};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
struct NodeServerImpl;
#[tonic::async_trait]
impl Node for NodeServerImpl {
    async fn get_node_info(
        &self,
        request: Request<GetNodeInfoRequest>,
    ) -> Result<Response<GetNodeInfoResponse>, Status> {
        Ok(Response::new(GetNodeInfoResponse::default()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let node_server = NodeServerImpl::default();

    Server::builder()
        .add_service(NodeServer::new(node_server))
        .serve(addr)
        .await?;

    Ok(())
}
