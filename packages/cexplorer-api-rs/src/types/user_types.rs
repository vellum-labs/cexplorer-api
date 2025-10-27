use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub name: String,
    pub picture: String,
    pub social: UserSocial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSocial {
    pub web: String,
    pub xcom: String,
    pub telegram: String,
    pub discord: String,
    pub patreon: String,
    pub facebook: String,
    pub instagram: String,
    pub github: String,
    pub linkedin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub profile: UserProfile,
    pub address: String,
    pub membership: UserMembership,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMembership {
    pub og: u64,
    pub nfts: u64,
    pub extra: Vec<String>,
}
