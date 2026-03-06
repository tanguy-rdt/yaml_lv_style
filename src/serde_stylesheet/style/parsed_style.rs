use std::collections::HashMap;

use serde::Deserialize;

use crate::serde_stylesheet::properties::Properties;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParsedStyle {
    #[serde(skip_deserializing)]
    pub name: String,
    #[serde(alias = "const", default = "Default::default")]
    const_style: bool,
    default: Option<Properties>,
    checked: Option<Properties>,
    focused: Option<Properties>,
    focus_key: Option<Properties>,
    edited: Option<Properties>,
    hovered: Option<Properties>,
    pressed: Option<Properties>,
    disabled: Option<Properties>,
    user_1: Option<Properties>,
    user_2: Option<Properties>,
    user_3: Option<Properties>,
    user_4: Option<Properties>,
    any: Option<Properties>,
}

impl ParsedStyle {
    pub fn is_const(&self) -> bool {
        self.const_style
    }

    pub fn is_empty(&self) -> bool {
        self.default.is_none()
            && self.checked.is_none()
            && self.focused.is_none()
            && self.focus_key.is_none()
            && self.edited.is_none()
            && self.hovered.is_none()
            && self.pressed.is_none()
            && self.disabled.is_none()
            && self.user_1.is_none()
            && self.user_2.is_none()
            && self.user_3.is_none()
            && self.user_4.is_none()
            && self.any.is_none()
    }

    pub fn into_properties_by_state_map(self) -> HashMap<String, Properties> {
        let mut map = HashMap::new();

        macro_rules! insert_if_some {
            ($field:ident, $state:expr) => {
                if let Some(value) = self.$field {
                    map.insert($state.to_string(), value);
                }
            };
        }

        insert_if_some!(default, "default");
        insert_if_some!(checked, "checked");
        insert_if_some!(focused, "focused");
        insert_if_some!(focus_key, "focus_key");
        insert_if_some!(edited, "edited");
        insert_if_some!(hovered, "hovered");
        insert_if_some!(pressed, "pressed");
        insert_if_some!(disabled, "disabled");
        insert_if_some!(user_1, "user_1");
        insert_if_some!(user_2, "user_2");
        insert_if_some!(user_3, "user_3");
        insert_if_some!(user_4, "user_4");
        insert_if_some!(any, "any");

        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_style_deserialization() {
        let yaml = r#"
            const: true
            default:
                width: 100
                align: center
            focused:
            hovered:
                width: 50
            "#;

        let parsed: ParsedStyle = yaml_serde::from_str(yaml).unwrap();

        assert!(parsed.is_const());
        assert!(parsed.default.is_some());
        assert!(parsed.focused.is_none());
        assert!(parsed.hovered.is_some());
    }

    #[test]
    fn test_parsed_style_deserialization_with_invalid_field() {
        let yaml = r#"
            const: true
            default:
                width: 100
            not_a_style:
                width: 100
            "#;

        let result: Result<ParsedStyle, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsed_style_deserialization_with_duplicate_style_state() {
        let yaml = r#"
            const: true
            default:
                width: 100
            default:
                width: 100
            "#;

        let result: Result<ParsedStyle, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_parsed_style_into_map() {
        let yaml = r#"
            const: true
            default:
                width: 100
                align: center
            focused:
            hovered:
                width: 50
            "#;

        let parsed: ParsedStyle = yaml_serde::from_str(yaml).unwrap();
        let map = parsed.into_properties_by_state_map();

        assert!(map.contains_key("default"));
        assert!(map.get("default").is_some());
        assert!(map.contains_key("hovered"));
        assert!(map.get("hovered").is_some());
        assert!(!map.contains_key("focused"));
    }
}
