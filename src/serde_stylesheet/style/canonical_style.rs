use std::ops::DerefMut;

use serde::Serialize;

use crate::serde_stylesheet::properties::PropertiesMap;
use crate::serde_stylesheet::style::ParsedStyle;
use crate::serde_stylesheet::types::AbstractType;

type PropertiesMapByState = (String, PropertiesMap);

#[derive(Serialize)]
pub struct CanonicalStyle {
    name: String,
    const_style: bool,
    properties_by_state: Vec<PropertiesMapByState>,
    declarations: Vec<String>,
}

impl CanonicalStyle {
    pub fn prepare_for_serialization(&mut self) {
        for properties_by_state in &mut self.properties_by_state {
            for property in properties_by_state.1.deref_mut() {
                let property_name = property.get_name().to_string();

                match property.get_value_mut() {
                    AbstractType::LVColor(color) => {
                        color.serialize_as_const(self.const_style);
                    }
                    AbstractType::LVImageColorkey(value) => {
                        value.serialize_as_const(self.const_style);
                    }
                    AbstractType::LVGridDscArray(value) => {
                        let declaration_name =
                            format!("{}_{}_{}", property_name, self.name, properties_by_state.0);
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
        let const_style = parsed_style.is_const();

        let properties_by_state = parsed_style
            .into_properties_by_state_map()
            .into_iter()
            .map(|(state, properties)| {
                let properties_map = PropertiesMap::from(properties);
                (state, properties_map)
            })
            .collect();

        Self {
            name,
            const_style,
            properties_by_state,
            declarations: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn yaml<'a>() -> &'a str {
        r#"
        name: test_style
        const: true
        default:
            width: 100
            align: center
        focused:
        hovered:
            width: 50
            grid_column_dsc_array: [px(100), fr(1)]
        "#
    }

    #[test]
    fn test_canonical_style_from_parsed_style() {
        let mut parsed: ParsedStyle = yaml_serde::from_str(yaml()).unwrap();
        parsed.name = "test_style".to_string();
        let canonical: CanonicalStyle = parsed.into();

        assert_eq!(canonical.name, "test_style");
        assert!(canonical.const_style);
        assert_eq!(canonical.properties_by_state.len(), 2);
        assert!(
            canonical
                .properties_by_state
                .iter()
                .any(|(s, _)| s == "default")
        );
        assert!(
            !canonical
                .properties_by_state
                .iter()
                .any(|(s, _)| s == "focused")
        );
        assert!(
            canonical
                .properties_by_state
                .iter()
                .any(|(s, _)| s == "hovered")
        );
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
