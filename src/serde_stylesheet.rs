mod lv_properties;
mod lv_types;
mod style;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use serde::Serialize;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer};

pub use lv_types::LVState;

use style::Style;

use crate::errors::Error;
use crate::errors::YamlLvStyleResult;

#[derive(Default, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StyleSheet {
    pub name: String,
    pub styles: Vec<Style>,
}

impl StyleSheet {
    pub fn from_yaml(path: &Path) -> YamlLvStyleResult<Self> {
        let yaml_str = fs::read_to_string(path).map_err(|e| Error::Io(e, path.to_path_buf()))?;

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::IoKind(io::ErrorKind::InvalidFilename, path.to_path_buf()))?;

        let mut stylesheet: StyleSheet = yaml_serde::from_str(&yaml_str)
            .map_err(|e| Error::from_yaml_serde(e, path.to_path_buf(), yaml_str.clone()))?;

        stylesheet.name = name.to_string();

        Ok(stylesheet)
    }
}

impl<'de> Deserialize<'de> for StyleSheet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_map = HashMap::<String, Style>::deserialize(deserializer)?;

        let mut styles = Vec::new();
        for (name, mut style) in raw_map {
            if style.is_empty() {
                return Err(DeError::custom(format!(
                    "Style '{}' is empty or invalid",
                    name
                )));
            }

            if let Some(const_style) = style.const_style
                && const_style
            {
                style.make_properties_const();
            }

            style.name = Some(name);
            styles.push(style);
        }

        if styles.is_empty() {
            return Err(DeError::custom("No styles defined in the stylesheet"));
        }

        Ok(StyleSheet {
            name: "default".to_string(),
            styles,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde_stylesheet::lv_types::LVAlign;

    #[test]
    fn test_stylesheet_deserialization() {
        let yaml = r#"
style_0:
  const: true
  default:
    width: 100
    align: center
  hovered:
    width: 50

style_1:
  const: true
  default:
    width: 100
    align: center
  hovered:
    width: 50
"#;

        let sheet: StyleSheet = yaml_serde::from_str(yaml).unwrap();

        assert_eq!(sheet.styles.len(), 2);
        for style in sheet.styles.iter() {
            assert!(style.name.as_ref().unwrap().starts_with("style_"));
            assert_eq!(style.const_style, Some(true));
            assert!(style.default.is_some());
            assert_eq!(style.default.as_ref().unwrap().width, Some(100));
            assert_eq!(style.default.as_ref().unwrap().align, Some(LVAlign::Center));
            assert!(style.hovered.is_some());
            assert_eq!(style.hovered.as_ref().unwrap().width, Some(50));
        }
    }
}
