use player_color_param::PlayerColorParam;

pub PlayerColorParamJson {
}

pub fn to_json(param: &PlayerColorParam) -> Result<String, Error> {
    return "placeholder".to_string();
}

pub fn from_json(json: &str) -> Result<PlayerColorParam, Error> {
    PlayerColorParam {
        entries: std::collections::HashMap::new(),
    }
}
