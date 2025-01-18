use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub uuid: String,
    pub host: String,
    pub port: String,
    pub coordinator_host: String,
    pub coordinator_port: String,
    pub ping_duration_seconds: u64,
    pub self_register_duration_seconds: u64,
}

static DEFAULT_HOST: &str = "127.0.0.1";
static DEFAULT_COORDINATOR_PORT: &str = "5151";

impl Config {

    pub fn load() -> Self {
        let unused_port = portpicker::pick_unused_port().expect("Failed to pick unused port");
        let random_uuid = uuid::Uuid::new_v4().to_string();

        let config = config::Config::builder()
            .add_source(config::File::with_name("config.yaml").required(false))
            .add_source(config::File::with_name("config.json").required(false))
            .add_source(config::Environment::default())

            .set_default("uuid", random_uuid).expect("Failed to set default uuid")
            .set_default("coordinator_host", DEFAULT_HOST).expect("Failed to set default address")
            .set_default("coordinator_port", DEFAULT_COORDINATOR_PORT).expect("Failed to set default address")
            .set_default("ping_duration_seconds", 5).expect("Failed to set default ping duration")
            .set_default("self_register_duration_seconds", 30).expect("Failed to set default self register duration")
            .set_default("host", DEFAULT_HOST).expect("Failed to set default address")
            .set_default("port", unused_port).expect("Failed to set default address")

            .build().expect("Failed to build configuration");

        
        let result = config.try_deserialize().expect("Failed to deserialize configuration");
        info!("Loaded configuration: {:?}", result);
        result
    }
}
