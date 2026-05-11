use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoInfo {
    pub name: String,
    pub owner: String,
    pub last_push: String,
}
