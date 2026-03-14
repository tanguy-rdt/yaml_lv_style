use crate::serde_stylesheet::types::LVColor;
use serde::Deserialize;
use serde::{Serialize, Serializer};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub struct LVImageColorkey {
    pub low: LVColor,
    pub high: LVColor,

    #[serde(skip)]
    declaration: String,
    #[serde(skip)]
    declaration_ref: String,
}

impl LVImageColorkey {
    pub fn make_declaration(&mut self, decl_name: &str) -> String {
        self.low.serialize_as_const(true);
        self.high.serialize_as_const(true);

        self.declaration_ref = format!("&{}", decl_name);
        self.declaration = format!(
            "static const lv_image_colorkey_t {} = {{{}, {}}};",
            decl_name,
            self.low.to_lvgl(),
            self.high.to_lvgl()
        );

        self.declaration.clone()
    }
}

impl Serialize for LVImageColorkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.declaration_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lv_image_colorkey_serde() {
        let yaml = r#"
            low: '#ffffff'
            high: rgb(255, 255, 255)
            "#;

        let mut parsed: LVImageColorkey = yaml_serde::from_str(yaml).unwrap();
        parsed.make_declaration("lv_image_colorkey_test");

        let out = yaml_serde::to_string(&parsed).unwrap();
        let out = out.trim_end().trim_matches('\'');

        let expected_declaration = "static const lv_image_colorkey_t lv_image_colorkey_test = {LV_COLOR_MAKE(255, 255, 255), LV_COLOR_MAKE(255, 255, 255)};";

        assert_eq!(out, "&lv_image_colorkey_test");
        assert_eq!(out, parsed.declaration_ref);
        assert_eq!(expected_declaration, parsed.declaration);
    }

    #[test]
    fn test_lv_image_colorkey_deserialization_invalid_value() {
        let yaml = r#"
            low: blue
            high: rgb(255, 255, 255)
            "#;
        let result: Result<LVImageColorkey, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());

        let yaml = r#"
            low: #ffffff
            low: rgb(255, 255, 255)
            "#;
        let result: Result<LVImageColorkey, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }
}
