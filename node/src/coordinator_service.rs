use crate::server::ServerConfiguration;

trait CoordinatorService {
    fn self_register(&self);
}

struct CoordinatorServiceImpl {
    server_configuration: ServerConfiguration,
}

impl CoordinatorService for CoordinatorServiceImpl {
    fn self_register(&self) {
        // Register the coordinator service to the coordinator
    }
}