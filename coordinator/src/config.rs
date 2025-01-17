use serde::Deserialize;
use tracing::info;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) address: String,
    pub(crate) ping_duration_seconds: u64,
}

static DEFAULT_ADDRESS: &str = "[::1]:50051";

impl Config {

    pub(crate) fn load() -> Self {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config.yaml").required(false))
            .add_source(config::File::with_name("config.json").required(false))
            .add_source(config::Environment::with_prefix("COORDINATOR"))

            .set_default("address", DEFAULT_ADDRESS).expect("Failed to set default address")
            .set_default("ping_duration_seconds", 5).expect("Failed to set default ping duration")

            .build().expect("Failed to build configuration");
        
        
        let result = config.try_deserialize().expect("Failed to deserialize configuration");
        info!("Loaded configuration: {:?}", result);
        result
    }
}
