mod properties;
mod style;
mod types;

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;

use serde::Serialize;

pub use types::LVState;

use style::CanonicalStyle;
use style::ParsedStyle;

use crate::errors::Error;
use crate::errors::YamlLvStyleResult;

#[derive(Default, Serialize)]
pub struct StyleSheet {
    pub name: String,
    pub styles: Vec<CanonicalStyle>,
}

impl StyleSheet {
    pub fn from_yaml(path: &Path) -> YamlLvStyleResult<Self> {
        let yaml_str = fs::read_to_string(path).map_err(|e| Error::Io(e, path.to_path_buf()))?;

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| Error::IoKind(io::ErrorKind::InvalidFilename, path.to_path_buf()))?;

        let parsed_styles = Self::deserialize_stylesheet(path, &yaml_str)?;

        let canonical_style: Vec<CanonicalStyle> = parsed_styles
            .into_iter()
            .map(|style| {
                let mut canonical_style = CanonicalStyle::from(style);
                canonical_style.prepare_for_serialization();
                canonical_style
            })
            .collect();

        Ok(StyleSheet {
            name: name.to_string(),
            styles: canonical_style,
        })
    }

    fn deserialize_stylesheet(path: &Path, yaml_str: &str) -> YamlLvStyleResult<Vec<ParsedStyle>> {
        let parsed_styles: Vec<ParsedStyle> =
            yaml_serde::from_str::<HashMap<String, ParsedStyle>>(yaml_str)
                .map_err(|e| Error::from_yaml_serde(e, path.to_path_buf(), yaml_str.to_string()))?
                .into_iter()
                .map(|(name, mut style)| {
                    style.name = name;

                    if style.is_empty() {
                        Err(Error::EmptyStyle(style.name, path.to_path_buf()))
                    } else {
                        Ok(style)
                    }
                })
                .collect::<Result<Vec<_>, _>>()?;
        Ok(parsed_styles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stylesheet_deserialization() {
        let yaml = r#"
            style_0:
              default:
                width: 100

            style_1:
              default:
                width: 100
            "#;

        let styles = StyleSheet::deserialize_stylesheet(Path::new("test.yaml"), yaml).unwrap();

        assert_eq!(styles.len(), 2);
        assert!(styles.iter().any(|(s)| s.name == "style_0"));
        assert!(styles.iter().any(|(s)| s.name == "style_1"));
    }

    #[test]
    fn test_stylesheet_deserialization_with_empty_style() {
        let yaml = r#"
            style_0:
              default:
                width: 100

            style_null:

            style_1:
              default:
                width: 100
            "#;

        let result = StyleSheet::deserialize_stylesheet(Path::new("test.yaml"), yaml);

        assert!(result.is_err());
    }
}
