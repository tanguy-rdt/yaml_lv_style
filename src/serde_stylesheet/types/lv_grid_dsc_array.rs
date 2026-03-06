use std::sync::OnceLock;

use regex::Regex;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

static PX_RE: OnceLock<Regex> = OnceLock::new();
static FR_RE: OnceLock<Regex> = OnceLock::new();

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
#[serde(transparent)]
pub struct LVGridDscArray {
    array: Vec<LVGridDscArrayValue>,

    #[serde(skip)]
    declaration: String,
    #[serde(skip)]
    declaration_ref: String,
}

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
enum LVGridDscArrayValue {
    Px(u32),
    Fr(u32),
}

impl LVGridDscArray {
    pub fn make_declaration(&mut self, decl_name: &str) -> String {
        let value = self
            .array
            .iter()
            .map(|v| match v {
                LVGridDscArrayValue::Px(px) => format!("{}", px),
                LVGridDscArrayValue::Fr(fr) => format!("LV_GRID_FR({})", fr),
            })
            .collect::<Vec<_>>()
            .join(", ");

        self.declaration_ref = decl_name.to_string();
        self.declaration = format!(
            "static const int32_t[] {} = {{{}, LV_GRID_TEMPLATE_LAST}};",
            decl_name, value
        );

        self.declaration.clone()
    }
}

impl<'de> Deserialize<'de> for LVGridDscArrayValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.trim();

        let px_re = PX_RE.get_or_init(|| {
            Regex::new(r"^px\(\s*(\d+)\s*\)$").expect("Failed to compile PX_RE regex")
        });

        if let Some(caps) = px_re.captures(s) {
            let value = caps[1].parse::<u32>().map_err(|_| {
                DeError::custom(format!(
                    "Invalid grid cell px value {}: use px(u32)",
                    &caps[1]
                ))
            })?;
            return Ok(LVGridDscArrayValue::Px(value));
        };

        let fr_re = FR_RE.get_or_init(|| {
            Regex::new(r"^(?:fr|lv_grid_fr)\(\s*(\d+)\s*\)$")
                .expect("Failed to compile FR_RE regex")
        });

        if let Some(caps) = fr_re.captures(s) {
            let value = caps[1].parse::<u32>().map_err(|_| {
                DeError::custom(format!(
                    "Invalid grid cell fr value {}: use fr(u32)",
                    &caps[1]
                ))
            })?;
            return Ok(LVGridDscArrayValue::Fr(value));
        };

        Err(DeError::custom(format!(
            "Invalid grid cell size: {s}, use px(u32) or fr(u32)"
        )))
    }
}

impl Serialize for LVGridDscArray {
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
    fn test_lv_grid_dsc_array_serde() {
        let yaml = r#"[px(100), px(50), fr(1)]"#;

        let mut parsed: LVGridDscArray = yaml_serde::from_str(yaml).unwrap();
        parsed.make_declaration("lv_grid_dsc_test");

        let out = yaml_serde::to_string(&parsed).unwrap();
        let out = out.trim_end();

        let expected_declaration = "static const int32_t[] lv_grid_dsc_test = {100, 50, LV_GRID_FR(1), LV_GRID_TEMPLATE_LAST};";

        assert_eq!(out, "lv_grid_dsc_test");
        assert_eq!(out, parsed.declaration_ref);
        assert_eq!(expected_declaration, parsed.declaration);
    }

    #[test]
    fn test_lv_grid_dsc_array_deserialization_invalid_value() {
        let yaml = r#"[100, 50, 1]"#;
        let result: Result<LVGridDscArray, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());

        let yaml = r#"px(100), px(50), fr(1)"#;
        let result: Result<LVGridDscArray, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());

        let yaml = r#"{px(100), px(50), fr(1)}"#;
        let result: Result<LVGridDscArray, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }
}
