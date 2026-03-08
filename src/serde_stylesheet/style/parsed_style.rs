use std::str::FromStr;

use serde::{Deserialize, Deserializer};

use crate::errors::Error;
use crate::serde_stylesheet::properties::Properties;
use crate::serde_stylesheet::types::LVSelector;

#[cfg_attr(test, derive(Debug))]
pub struct ParsedStyle {
    pub name: String,
    pub const_style: bool,
    pub properties_by_selector: Vec<(LVSelector, Properties)>,
}

impl<'de> Deserialize<'de> for ParsedStyle {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::{MapAccess, Visitor};

        struct ParsedStyleVisitor;

        impl<'de> Visitor<'de> for ParsedStyleVisitor {
            type Value = ParsedStyle;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "a ParsedStyle mapping")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut name: Option<String> = None;
                let mut const_style: bool = false;
                let mut properties_by_selector: Vec<(LVSelector, Properties)> = Vec::new();

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "name" => {
                            name = Some(map.next_value()?);
                        }
                        "const" => {
                            const_style = map.next_value()?;
                        }
                        selector_str => {
                            let selector = LVSelector::from_str(selector_str)
                                .map_err(serde::de::Error::custom)?;
                            let properties: Option<Properties> = map.next_value()?;

                            let properties = properties.ok_or_else(|| {
                                serde::de::Error::custom(Error::EmptySelector(
                                    selector.to_lv().clone(),
                                ))
                            })?;

                            if properties_by_selector.iter().any(|(s, _)| s == &selector) {
                                return Err(serde::de::Error::custom(Error::DuplicatedSelector(
                                    selector.to_lv().clone(),
                                )));
                            }

                            properties_by_selector.push((selector, properties));
                        }
                    }
                }

                Ok(ParsedStyle {
                    name: name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    const_style,
                    properties_by_selector,
                })
            }
        }

        deserializer.deserialize_map(ParsedStyleVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::serde_stylesheet::types::{LVPart, LVState};

    #[test]
    fn test_parsed_style_deserialization() {
        let yaml = r#"
            name: test_style
            const: true
            default:
                width: 100
                align: center
            hovered:
                width: 50
            default.indicator:
                width: 200
            user_1.indicator:
                width: 300
            "#;

        let parsed: ParsedStyle = yaml_serde::from_str(yaml).unwrap();

        assert!(parsed.const_style);
        assert_eq!(parsed.name, "test_style");

        let default = parsed
            .properties_by_selector
            .iter()
            .find(|(s, _)| s.part == LVPart::Main && s.state == LVState::Default);
        assert!(default.is_some());

        let hovered = parsed
            .properties_by_selector
            .iter()
            .find(|(s, _)| s.part == LVPart::Main && s.state == LVState::Hovered);
        assert!(hovered.is_some());

        let indicator_default = parsed
            .properties_by_selector
            .iter()
            .find(|(s, _)| s.part == LVPart::Indicator && s.state == LVState::Default);
        assert!(indicator_default.is_some());

        let indicator_user1 = parsed
            .properties_by_selector
            .iter()
            .find(|(s, _)| s.part == LVPart::Indicator && s.state == LVState::User1);
        assert!(indicator_user1.is_some());
    }

    #[test]
    fn test_parsed_style_deserialization_with_no_properties() {
        let yaml = r#"
            name: test_style
            const: true
            default:
                width: 100
            focused:
            hovered:
                width: 200
            "#;

        let result: Result<ParsedStyle, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsed_style_deserialization_with_invalid_selector() {
        let yaml = r#"
            name: test_style
            const: true
            default:
                width: 100
            not_a_state:
                width: 100
            "#;

        let result: Result<ParsedStyle, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsed_style_deserialization_with_invalid_part() {
        let yaml = r#"
            name: test_style
            const: true
            default.not_a_part:
                width: 100
            "#;

        let result: Result<ParsedStyle, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsed_style_deserialization_with_duplicate_selector() {
        let yaml = r#"
            name: test_style
            const: true
            default:
                width: 100
            default:
                width: 200
            "#;

        let result: Result<ParsedStyle, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsed_style_default_const_is_false() {
        let yaml = r#"
            name: test_style
            default:
                width: 100
            "#;

        let parsed: ParsedStyle = yaml_serde::from_str(yaml).unwrap();
        assert!(!parsed.const_style);
    }
}
