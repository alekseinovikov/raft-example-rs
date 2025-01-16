use api::api::coordinator_client::CoordinatorClient;

pub(crate) struct Coordinator {
    client: CoordinatorClient<tonic::transport::Channel>,
}

impl Coordinator {
    pub(crate) async fn new(host: &str, port: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client
            = CoordinatorClient::connect(format!("http://{host}:{port}")).await?;
        Ok(Self { client })
    }

}