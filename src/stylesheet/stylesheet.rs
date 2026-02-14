use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Serialize;
use serde::{Deserialize, Deserializer};

use super::style::Style;

#[derive(Debug, Default)]
#[derive(Serialize)]
pub struct StyleSheet {
    pub name: String,
    pub styles: Vec<Style>,
}

impl StyleSheet {
    pub fn from_yaml(path: &Path) -> Result<Self, String> {
        let name = path.file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| format!("Invalid file name: {}", path.display()))?;

        let yaml_str = fs::read_to_string(path)
            .map_err(|e| format!("Unable to read file '{}': {}", path.display(), e))?;

        let mut stylesheet: StyleSheet = yaml_serde::from_str(&yaml_str)
            .map_err(|e| format!("YAML parsing error '{}': {}", path.display(), e))?;

        stylesheet.name = name.to_string();

        Ok(stylesheet)
    }
}

impl<'de> Deserialize<'de> for StyleSheet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_map = HashMap::<String, Vec<Style>>::deserialize(deserializer)?;

        let styles = raw_map.into_iter().map(|(style_name, elements)| {
            let mut final_style = Style {
                name: Some(style_name),
                ..Default::default()
            };

            for element in elements {
                if let Some(c) = element.const_style { final_style.const_style = Some(c); }
                if let Some(s) = element.default { final_style.default = Some(s); }
                if let Some(s) = element.checked { final_style.checked = Some(s); }
                if let Some(s) = element.focused { final_style.focused = Some(s); }
                if let Some(s) = element.focus_key { final_style.focus_key = Some(s); }
                if let Some(s) = element.edited { final_style.edited = Some(s); }
                if let Some(s) = element.hovered { final_style.hovered = Some(s); }
                if let Some(s) = element.pressed { final_style.pressed = Some(s); }
                if let Some(s) = element.disabled { final_style.disabled = Some(s); }
                if let Some(s) = element.user_1 { final_style.user_1 = Some(s); }
                if let Some(s) = element.user_2 { final_style.user_2 = Some(s); }
                if let Some(s) = element.user_3 { final_style.user_3 = Some(s); }
                if let Some(s) = element.user_4 { final_style.user_4 = Some(s); }
                if let Some(s) = element.any { final_style.any = Some(s); }
            }
            final_style
        }).collect();


        Ok(StyleSheet {
            name: String::new(), // Will be filled in from_yaml fn
            styles
        })
    }
}