use player_color_param::{PlayerColorParam, EntryKey, RGB};
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

#[derive(Serialize, Deserialize)]
struct PlayerColorParamJson {
    #[serde(default = "api_filetype")]
    filetype: String,
    version: u32,
    #[serde(flatten)]
    entries: IndexMap<String, String>,
}

fn api_filetype() -> String {
    "PlayerColorParam".to_string()
}

pub fn from_json(json: &str) -> Result<PlayerColorParam, serde_json::Error> {
    let layout: PlayerColorParamJson = serde_json::from_str(json)?;
    Ok(PlayerColorParam {
        entries: layout.entries.iter()
            .filter_map(|(key, val)| Some((string_to_key(key)?, RGB::from_hex_str(val)?)))
            .collect(),
    })
}

pub fn to_json(param: &PlayerColorParam) -> Result<String, serde_json::Error> {
    let layout = PlayerColorParamJson {
        filetype: api_filetype(),
        version: 260218,
        entries: param.entries.iter()
            .map(|(key, rgb)| (key_to_string(key), rgb.to_hex_str(true)))
            .collect(),
    };
    let json = serde_json::to_string_pretty(&layout)?;

    let mut lines: Vec<&str> = json.lines().collect();
    if lines.len() > 2 {
        lines.insert(3, "");
    }

    Ok(lines.join("\n"))
}

fn string_to_key(s: &str) -> Option<EntryKey> {
    if s.len() != 10 {
        return None;
    }

    let character_id = format!("{}0{}", &s[0..4], &s[5..6]);
    let costume_index = s.as_bytes()[4].checked_sub(b'0')?;
    let alt_index = s.as_bytes()[9].checked_sub(b'0')?;

    if costume_index > 9 || alt_index > 9 {
        return None;
    }

    Some(EntryKey {
        character_id,
        costume_index,
        alt_index,
    })
}

fn key_to_string(key: &EntryKey) -> String {
    format!("{}{}{}col{}", &key.character_id[0..4], key.costume_index, &key.character_id[5..6], key.alt_index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use player_color_param::{IndexMap, EntryKey, RGB, PlayerColorParam};

    #[test]
    fn test_two_way() {
        let mut entries = IndexMap::new();
        let entry_key = EntryKey {
            character_id: "5grn01".to_string(),
            costume_index: 2,
            alt_index: 3,
        };
        entries.insert(
            entry_key.clone(),
            RGB { red: 255, green: 0, blue: 69 },
        );

        let param = PlayerColorParam { entries };
        let json = to_json(&param).unwrap();

        println!("Generated JSON:\n{}", json);
        assert!(json.contains("5grn21col3"));
        assert!(json.contains("#FF0045"));

        let param = from_json(&json).unwrap();

        assert!(param.entries.contains_key(&entry_key));
        assert_eq!(param.entries[&entry_key].red, 255);
        assert_eq!(param.entries[&entry_key].green, 0);
        assert_eq!(param.entries[&entry_key].blue, 69);
    }
}
