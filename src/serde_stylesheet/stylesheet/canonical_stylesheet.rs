use serde::Serialize;

use crate::serde_stylesheet::ParsedStyleSheet;
use crate::serde_stylesheet::style::CanonicalStyle;

#[derive(Serialize)]
pub struct CanonicalStyleSheet {
    name: String,
    styles: Vec<CanonicalStyle>,
}

impl CanonicalStyleSheet {
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl From<ParsedStyleSheet> for CanonicalStyleSheet {
    fn from(parsed_stylesheet: ParsedStyleSheet) -> Self {
        let canonical_styles: Vec<CanonicalStyle> = parsed_stylesheet
            .styles
            .into_iter()
            .map(|style| {
                let mut canonical: CanonicalStyle = style.into();
                canonical.prepare_for_serialization();
                canonical
            })
            .collect();
        CanonicalStyleSheet {
            name: parsed_stylesheet.name,
            styles: canonical_styles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_canonical_stylesheet_from_parsed_stylesheet() {
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
        let canonical_stylesheet: CanonicalStyleSheet = parsed_stylesheet.into();

        assert_eq!(canonical_stylesheet.name, "test_stylesheet");
        assert_eq!(canonical_stylesheet.styles.len(), 2);
        assert!(
            canonical_stylesheet
                .styles
                .iter()
                .any(|s| s.get_name() == "style_0")
        );
        assert!(
            canonical_stylesheet
                .styles
                .iter()
                .any(|s| s.get_name() == "style_1")
        );
    }
}
