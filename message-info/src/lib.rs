use message_info::MessageInfo;
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

#[derive(Serialize, Deserialize)]
struct MessageInfoJson {
    #[serde(default = "api_filetype")]
    filetype: String,
    version: u32,
    language: String,
    #[serde(flatten)]
    entries: IndexMap<String, Entry>,
}

fn api_filetype() -> String {
    "MessageInfo".to_string()
}

#[derive(Serialize, Deserialize)]
struct Entry {
    #[serde(default = "api_filetype")]
}

pub fn from_json(json: &str) -> Result<MessageInfo, serde_json::Error> {
    let layout: MessageInfoJson serde_json::from_str(json)?;
    Ok(MessageInfo {
        entries: layout.entries.iter()
            .filter_map(|(key, val)|
                Some((string_to_key(key)?, RGB::from_hex_str(val)?))
            )
            .collect(),
    })
}
