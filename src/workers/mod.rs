pub mod client;
pub mod search;

pub struct Worker {
    client: reqwest::Client,
}

impl Default for Worker {
    fn default() -> Self {
        Worker {
            client: client::get_client(),
        }
    }
}
