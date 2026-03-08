use std::collections::HashSet;
use std::io;
use std::path::Path;

use serde::Deserialize;

use crate::errors::Error;
use crate::errors::Result;
use crate::serde_stylesheet::style::ParsedStyle;

#[derive(Deserialize)]
pub struct ParsedStyleSheet {
    #[serde(default)]
    pub name: String,
    pub styles: Vec<ParsedStyle>,
}

impl ParsedStyleSheet {
    pub fn deserialize_stylesheet(path: &Path, yaml_str: &str) -> Result<ParsedStyleSheet> {
        let mut parsed_stylesheet: ParsedStyleSheet = yaml_serde::from_str(yaml_str)
            .map_err(|e| Error::yaml_serde(e, path.to_path_buf(), yaml_str.to_string()))?;

        let mut seen_names = HashSet::new();
        for style in &mut parsed_stylesheet.styles {
            if style.properties_by_selector.is_empty()
                && style.name != "null"
                && style.name != "None"
            {
                return Err(Error::EmptyStyle(style.name.clone(), path.to_path_buf()));
            }

            if !seen_names.insert(&style.name) {
                return Err(Error::DuplicatedStyle(
                    style.name.clone(),
                    path.to_path_buf(),
                ));
            }
        }

        if parsed_stylesheet.name.is_empty() {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| Error::IoKind(io::ErrorKind::InvalidFilename, path.to_path_buf()))?;
            parsed_stylesheet.name = name.to_string();
        }

        Ok(parsed_stylesheet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_stylesheet_deserialization() {
        let yaml = r#"
            name: test_stylesheet
            styles:
              - name: style_0
                default:
                  width: 100

              - name: style_1
                default:
                  width: 100
            "#;

        let parsed_stylesheet =
            ParsedStyleSheet::deserialize_stylesheet(Path::new("test.yaml"), &yaml).unwrap();

        assert_eq!(parsed_stylesheet.styles.len(), 2);
        assert_eq!(parsed_stylesheet.name, "test_stylesheet".to_string());
        assert!(parsed_stylesheet.styles.iter().any(|s| s.name == "style_0"));
        assert!(parsed_stylesheet.styles.iter().any(|s| s.name == "style_1"));
    }

    #[test]
    fn test_parsed_stylesheet_deserialization_with_empty_style() {
        let yaml = r#"
            name: test_stylesheet
            styles:
              - name: style_0
                default:
                  width: 100

              - name: style_null

              - name: style_1
                default:
                  width: 100
            "#;

        let result: std::result::Result<ParsedStyleSheet, _> =
            ParsedStyleSheet::deserialize_stylesheet(Path::new("test.yaml"), &yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsed_stylesheet_deserialization_with_duplicate_style() {
        let yaml = r#"
            name: test_stylesheet
            styles:
              - name: style_0
                default:
                  width: 100

              - name: style_0
                default:
                  width: 100
            "#;

        let result: std::result::Result<ParsedStyleSheet, _> =
            ParsedStyleSheet::deserialize_stylesheet(Path::new("test.yaml"), &yaml);
        assert!(result.is_err());
    }
}
