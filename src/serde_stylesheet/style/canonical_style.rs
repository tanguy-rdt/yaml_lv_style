use std::ops::DerefMut;

use serde::Serialize;

use crate::serde_stylesheet::properties::PropertiesMap;
use crate::serde_stylesheet::style::ParsedStyle;
use crate::serde_stylesheet::types::{AbstractType, LVSelector};

#[derive(Serialize)]
pub struct CanonicalStyle {
    name: String,
    const_style: bool,
    properties_by_selector: Vec<(LVSelector, PropertiesMap)>,
    declarations: Vec<String>,
}

impl CanonicalStyle {
    pub fn prepare_for_serialization(&mut self) {
        for properties_by_selector in &mut self.properties_by_selector {
            for property in properties_by_selector.1.deref_mut() {
                let property_name = property.get_name().to_string();

                match property.get_value_mut() {
                    AbstractType::LVColor(color) => {
                        color.serialize_as_const(self.const_style);
                    }
                    AbstractType::LVImageColorkey(value) => {
                        let declaration_name = format!(
                            "{}_{}_{}",
                            property_name,
                            self.name,
                            &properties_by_selector.0.to_snake_case()
                        );
                        self.declarations
                            .push(value.make_declaration(&declaration_name));
                    }
                    AbstractType::LVGridDscArray(value) => {
                        let declaration_name = format!(
                            "{}_{}_{}",
                            property_name,
                            self.name,
                            &properties_by_selector.0.to_snake_case()
                        );
                        self.declarations
                            .push(value.make_declaration(&declaration_name));
                    }
                    _ => (),
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl From<ParsedStyle> for CanonicalStyle {
    fn from(parsed_style: ParsedStyle) -> Self {
        let name = parsed_style.name.clone();
        let const_style = parsed_style.const_style;

        let properties_by_selector = parsed_style
            .properties_by_selector
            .into_iter()
            .map(|(state, properties)| {
                let properties_map = PropertiesMap::from(properties);
                (state, properties_map)
            })
            .collect();

        Self {
            name,
            const_style,
            properties_by_selector,
            declarations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serde_stylesheet::types::{LVPart, LVState};

    fn yaml<'a>() -> &'a str {
        r#"
        name: test_style
        const: true
        default:
            grid_column_dsc_array: [px(100), fr(1)]
            width: 100
            align: center
        hovered:
            width: 50
        default.indicator:
            width: 200
        user_1.indicator:
            width: 300
        "#
    }

    #[test]
    fn test_canonical_style_from_parsed_style() {
        let mut parsed: ParsedStyle = yaml_serde::from_str(yaml()).unwrap();
        parsed.name = "test_style".to_string();
        let canonical: CanonicalStyle = parsed.into();

        assert_eq!(canonical.name, "test_style");
        assert!(canonical.const_style);
        assert_eq!(canonical.properties_by_selector.len(), 4);

        let default = canonical
            .properties_by_selector
            .iter()
            .find(|(s, _)| s.part == LVPart::Main && s.state == LVState::Default);
        assert!(default.is_some());

        let hovered = canonical
            .properties_by_selector
            .iter()
            .find(|(s, _)| s.part == LVPart::Main && s.state == LVState::Hovered);
        assert!(hovered.is_some());

        let indicator_default = canonical
            .properties_by_selector
            .iter()
            .find(|(s, _)| s.part == LVPart::Indicator && s.state == LVState::Default);
        assert!(indicator_default.is_some());

        let indicator_user1 = canonical
            .properties_by_selector
            .iter()
            .find(|(s, _)| s.part == LVPart::Indicator && s.state == LVState::User1);
        assert!(indicator_user1.is_some());
    }

    #[test]
    fn test_prepare_canonical_style_for_serialization() {
        let mut parsed: ParsedStyle = yaml_serde::from_str(yaml()).unwrap();
        parsed.name = "test_style".to_string();
        let mut canonical: CanonicalStyle = parsed.into();
        canonical.prepare_for_serialization();

        assert_eq!(canonical.declarations.len(), 1);
    }
}
