use std::collections::HashMap;
use std::fs;

use serde::{Deserialize, Serialize};

use super::lv_properties::LVProperties;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Style {
    pub name: String,
    pub const_style: Option<bool>,
    pub default: Option<LVProperties>,
    pub checked: Option<LVProperties>,
    pub focused: Option<LVProperties>,
    pub focus_key: Option<LVProperties>,
    pub edited: Option<LVProperties>,
    pub hovered: Option<LVProperties>,
    pub pressed: Option<LVProperties>,
    pub disabled: Option<LVProperties>,
    pub user_1: Option<LVProperties>,
    pub user_2: Option<LVProperties>,
    pub user_3: Option<LVProperties>,
    pub user_4: Option<LVProperties>,
    pub any: Option<LVProperties>,
}

#[derive(Debug, Default)]
pub struct StyleSheet {
    pub styles: Vec<Style>,
}

impl StyleSheet {
    pub fn from_yaml(path: &str) -> Result<Self, String> {
        let yaml_str = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => return Err(format!("Unable to read the YAML file '{}': {}", path, e)),
        };

        let raw: HashMap<String, Vec<HashMap<String, yaml_serde::Value>>> = match yaml_serde::from_str(&yaml_str) {
            Ok(r) => r,
            Err(e) => return Err(format!("YAML parsing error '{}': {}", path, e)),
        };

        let mut styles = Vec::new();
        for (name, vec_maps) in raw {
            let mut style = Style {
                name,
                ..Default::default()
            };

            for map in vec_maps {
                for (key, value) in map {
                    if key.as_str() == "const" {
                        style.const_style = match yaml_serde::from_value(value) {
                            Ok(b) => Some(b),
                            Err(e) => {
                                log::warn!("Unable to read Boolean value for 'const': {}", e);
                                None
                            }
                        };
                    } else {
                        match key.as_str() {
                            "default" => {
                                style.default = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “default” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "checked" => {
                                style.checked = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “checked” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "focused" => {
                                style.focused = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “focused” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "focus_key" => {
                                style.focus_key = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “focus_key” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "edited" => {
                                style.edited = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “edited” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "hovered" => {
                                style.hovered = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “hovered” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "pressed" => {
                                style.pressed = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “pressed” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "disabled" => {
                                style.disabled = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “disabled” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "user_1" => {
                                style.user_1 = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “user_1” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "user_2" => {
                                style.user_2 = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “user_2” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "user_3" => {
                                style.user_3 = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “user_3” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "user_4" => {
                                style.user_4 = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “user_4” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            "any" => {
                                style.any = match yaml_serde::from_value(value) {
                                    Ok(p) => Some(p),
                                    Err(e) => {
                                        log::warn!("Unable to read state style “any” for “{}”: {}", style.name, e);
                                        None
                                    }
                                };
                            }
                            other => {
                                log::warn!("Key “{}” not supported in style '{}'", other, style.name);
                            }
                        }

                    }
                }
            }
            styles.push(style);
        }

        Ok(StyleSheet { styles })
    }
}