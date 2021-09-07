pub mod remote;
pub mod local;

use serde_derive::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Tabled, Debug, Clone)]
pub struct RemoteRepoRegistry {
    pub name: String,
    pub url: String,
    pub requires_authorization: bool,
}

#[derive(Serialize, Deserialize)]
pub struct LocalRepoRegistry {
    pub name: String,
    pub path: String,
}