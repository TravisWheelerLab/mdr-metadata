use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedJson {
    pub total_filenum: u32,
    pub total_filesize: u64,
    pub token: Option<String>,
    pub status: String,
    pub files: Vec<CompletedJsonFile>,
    pub time: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompletedJsonFile {
    pub irods_path: String,
    pub size: u64,
    pub md5_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MdrepoTicket {
    pub id: u32,
    pub created_at: String,
    pub token: String,
    pub full_token: String,
    pub irods_tickets: String,
}
