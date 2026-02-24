use message_info::{MessageInfo, hash, FromStr};
use serde::{Deserialize, Serialize};
use indexmap::IndexMap;

#[derive(Serialize, Deserialize)]
struct MessageInfoJson {
    #[serde(default = "api_filetype")]
    filetype: String,
    version: u32,
    language: String,
    colors: IndexMap<String, String>,
    #[serde(flatten)]
    entries: IndexMap<String, Entry>,
}

fn api_filetype() -> String {
    "MessageInfo".to_string()
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct Entry {
    message: Option<String>,
    reference: Option<String>,
    adx2_file: Option<String>,
    adx2_cue_index: Option<u16>,
}

pub fn from_json(json: &str) -> Result<MessageInfo, serde_json::Error> {
    let mut layout: MessageInfoJson = serde_json::from_str(json)?;

    for val in layout.colors.values_mut() {
        if val.starts_with('#') {
            *val = val[1..].to_string();
        }
    }

    Ok(MessageInfo {
        language: message_info::Language::from_str(&layout.language).unwrap(),
        entries: layout.entries.into_iter()
            .filter_map(|(key, val)| {
                let (hash_id, string_id) = match u32::from_str_radix(&key, 16) {
                    Ok(id) => (id, None),
                    Err(_) => (hash(&key), Some(key.clone())),
                };

                let message = val.message.map(|msg| {
                    let mut result = msg;
                    for (key, val) in &layout.colors {
                        result = result
                            .replace(&format!("<{}>", key), &format!("<color 0x{}>", val))
                            .replace(&format!("</{}>", key), "</color>");
                        }
                    result
                });

                let reference_id = val.reference.map(|reference|
                    match u32::from_str_radix(&reference, 16) {
                        Ok(id) => message_info::Reference::HashId(id),
                        Err(_) => message_info::Reference::StringId(reference),
                    }
                );

                Some((
                    hash_id,
                    message_info::Entry {
                        string_id,
                        message,
                        reference_id,
                        adx2_file: val.adx2_file,
                        adx2_cue_index: val.adx2_cue_index,
                    }
                ))
            }).collect(),
    })
}

pub fn to_json(param: &MessageInfo) -> Result<String, serde_json::Error> {
    let layout = MessageInfoJson {
        filetype: api_filetype(),
        version: 260224,
        language: param.language.to_string(),
        colors: IndexMap::new(),
        entries: param.entries.iter()
            .map(|(key, val)| {
                let json_key = if let Some(id) = val.string_id.clone() {
                    id
                } else {
                    format!("{:08x}", key)
                };

                let reference = val.reference_id.clone().map(|reference|
                    match reference {
                        message_info::Reference::StringId(id) => id,
                        message_info::Reference::HashId(id) => format!("{:08x}", id),
                    }
                );
                let entry = Entry {
                    message: val.message.clone(),
                    reference,
                    adx2_file: val.adx2_file.clone(),
                    adx2_cue_index: val.adx2_cue_index.clone(),
                };
                (json_key, entry)
            }).collect(),
    };
    let json = serde_json::to_string_pretty(&layout)?;

    let mut lines: Vec<&str> = json.lines().collect();
    if lines.len() > 4 {
        lines.insert(5, "");
    }

    Ok(lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use message_info::{IndexMap, MessageInfo, hash};

    #[test]
    fn test_two_way() {
        let mut entries = IndexMap::new();
        let key = "3jtr01_btlst_00_3dio01".to_string();
        let hashed_key = hash(&key);
        let message = "Imma beat you up cuh".to_string();
        let adx2_file = "v_btl_3jtr01".to_string();
        entries.insert(
            hashed_key.clone(),
            message_info::Entry {
                string_id: Some(key.clone()),
                message: Some(message.clone()),
                reference_id: None,
                adx2_file: Some(adx2_file.clone()),
                adx2_cue_index: Some(33),
            },
        );

        let param = MessageInfo {
            language: message_info::Language::English,
            entries,
        };
        let json = to_json(&param).unwrap();

        println!("Generated JSON:\n{}", json);
        assert!(json.contains(&key));

        let param = from_json(&json).unwrap();

        assert!(param.entries.contains_key(&hashed_key));
        assert_eq!(param.entries[&hashed_key].string_id, Some(key));
        assert_eq!(param.entries[&hashed_key].message, Some(message));
        assert_eq!(param.entries[&hashed_key].adx2_file, Some(adx2_file));
        assert_eq!(param.entries[&hashed_key].adx2_cue_index, Some(33));
        assert!(!param.entries[&hashed_key].reference_id.is_some());
    }
}
