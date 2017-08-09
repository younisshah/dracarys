#[derive(Debug)]
pub enum DracarysError {
    ADDError(&'static str)
}

/// KongAPI represents POST body for
/// adding an API in Kong
#[derive(Debug, Serialize, Deserialize)]
pub struct KongAPI {
    pub name: String,
    pub hosts: Vec<String>,
    pub uris: Vec<String>,
    pub preserve_host: bool,
    pub upstream_url: String,
    pub https_only: bool,
    pub http_if_terminated: bool,
}