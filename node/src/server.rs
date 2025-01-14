use portpicker::pick_unused_port;

pub(crate) struct ServerConfiguration {
    port: u16,
}

impl ServerConfiguration {
    pub(crate) fn new_with_random_port() -> Self {
        let free_port = pick_unused_port().expect("No available port is found!");
        Self { port: free_port }
    }
}
