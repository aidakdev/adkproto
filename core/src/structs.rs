use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub type DID = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub did: DID,
    pub handle: String,
    pub display_name: String,
    pub public_key: String,
    pub created_at: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub author: DID,
    pub content: String,
    pub created_at: u64,
    pub signature: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Follow {
    pub follower: DID,
    pub following: DID,
    pub created_at: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FederationEvent {
    UserCreated(User),
    PostCreated(Post),
    FollowCreated(Follow)
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederationMessage {
    pub id: String,
    pub event: FederationEvent,
    pub timestamp: u64,
    pub source_server: String,
    pub signature: String
}

pub fn current_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}



