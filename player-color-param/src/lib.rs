use player_color_param::PlayerColorParam;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct PlayerColorParamJson {
    #[serde(default = "api_filetype")]
    filetype: String,
    version: u32,
    #[serde(flatten)]
    entries: HashMap<String, String>,
}

fn api_filetype() -> String {
    "PlayerColorParam".to_string()
}

pub fn to_json(param: &PlayerColorParam) -> Result<String, serde_json::Error> {
    let layout = PlayerColorParamJson {
        filetype: api_filetype(),
        version: 260218,
        entries: param.entries.iter()
            .map(|(key, rgb)| (key_to_string(key), rgb.to_hex_str(true)))
            .collect(),
    };
    serde_json::to_string_pretty(&layout)
}

pub fn from_json(json: &str) -> Result<PlayerColorParam, serde_json::Error> {
    let layout: PlayerColorParamJson = serde_json::from_str(json)?;
    Ok(PlayerColorParam {
        entries: layout.entries.iter()
            .filter_map(|(key, val)| Some((string_to_key(key)?, player_color_param::RGB::from_hex_str(val)?)))
            .collect(),
    })
}

fn string_to_key(s: &str) -> Option<player_color_param::EntryKey> {
    if s.len() != 10 {
        return None;
    }

    let character_id = format!("{}0{}", &s[0..4], &s[5..6]);
    let costume_index = s.as_bytes()[4].checked_sub(b'0')?;
    let alt_index = s.as_bytes()[9].checked_sub(b'0')?;

    if costume_index > 9 || alt_index > 9 {
        return None;
    }

    Some(player_color_param::EntryKey {
        character_id,
        costume_index,
        alt_index,
    })
}

fn key_to_string(key: &player_color_param::EntryKey) -> String {
    format!("{}{}{}col{}", &key.character_id[0..4], key.costume_index, &key.character_id[5..6], key.alt_index)
}
