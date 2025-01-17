use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub uuid: String,
    pub address: String,
    pub coordinator_address: String,
    pub ping_duration_seconds: u64,
}

static DEFAULT_COORDINATOR_ADDRESS: &str = "[::1]:50051";

impl Config {

    pub fn load() -> Self {
        let unused_port = portpicker::pick_unused_port().expect("Failed to pick unused port");
        let my_address = format!("[::1]:{}", unused_port.to_string());
        let random_uuid = uuid::Uuid::new_v4().to_string();

        let config = config::Config::builder()
            .add_source(config::File::with_name("config.yaml").required(false))
            .add_source(config::File::with_name("config.json").required(false))
            .add_source(config::Environment::default())

            .set_default("uuid", random_uuid).expect("Failed to set default uuid")
            .set_default("coordinator_address", DEFAULT_COORDINATOR_ADDRESS).expect("Failed to set default address")
            .set_default("ping_duration_seconds", 5).expect("Failed to set default ping duration")
            .set_default("address", my_address).expect("Failed to set default address")

            .build().expect("Failed to build configuration");

        
        let result = config.try_deserialize().expect("Failed to deserialize configuration");
        info!("Loaded configuration: {:?}", result);
        result
    }
}
