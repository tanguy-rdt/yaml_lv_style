use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, strum_macros::EnumIter))]
#[derive(PartialEq, Deserialize, Serialize)]
pub enum LVPart {
    #[serde(rename = "LV_PART_MAIN", alias = "main")]
    Main,
    #[serde(rename = "LV_PART_SCROLLBAR", alias = "scrollbar")]
    Scrollbar,
    #[serde(rename = "LV_PART_INDICATOR", alias = "indicator")]
    Indicator,
    #[serde(rename = "LV_PART_KNOB", alias = "knob")]
    Knob,
    #[serde(rename = "LV_PART_SELECTED", alias = "selected")]
    Selected,
    #[serde(rename = "LV_PART_CURSOR", alias = "cursor")]
    Cursor,
    #[serde(rename = "LV_PART_CUSTOM_FIRST", alias = "custom_first")]
    CustomFirst,
    #[serde(rename = "LV_PART_ANY", alias = "any")]
    Any,
}

impl LVPart {
    pub fn to_snake_case(&self) -> &str {
        match self {
            LVPart::Main => "main",
            LVPart::Scrollbar => "scrollbar",
            LVPart::Indicator => "indicator",
            LVPart::Knob => "knob",
            LVPart::Selected => "selected",
            LVPart::Cursor => "cursor",
            LVPart::CustomFirst => "custom_first",
            LVPart::Any => "any",
        }
    }
}

impl FromStr for LVPart {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "main" | "LV_PART_MAIN" => Ok(LVPart::Main),
            "scrollbar" | "LV_PART_SCROLLBAR" => Ok(LVPart::Scrollbar),
            "indicator" | "LV_PART_INDICATOR" => Ok(LVPart::Indicator),
            "knob" | "LV_PART_KNOB" => Ok(LVPart::Knob),
            "selected" | "LV_PART_SELECTED" => Ok(LVPart::Selected),
            "cursor" | "LV_PART_CURSOR" => Ok(LVPart::Cursor),
            "custom_first" | "LV_PART_CUSTOM_FIRST" => Ok(LVPart::CustomFirst),
            "any" | "LV_PART_ANY" => Ok(LVPart::Any),
            _ => Err(format!("Unknown LVPart: '{value}'")),
        }
    }
}

impl From<&LVPart> for String {
    fn from(part: &LVPart) -> Self {
        match part {
            LVPart::Main => "LV_PART_MAIN".to_string(),
            LVPart::Scrollbar => "LV_PART_SCROLLBAR".to_string(),
            LVPart::Indicator => "LV_PART_INDICATOR".to_string(),
            LVPart::Knob => "LV_PART_KNOB".to_string(),
            LVPart::Selected => "LV_PART_SELECTED".to_string(),
            LVPart::Cursor => "LV_PART_CURSOR".to_string(),
            LVPart::CustomFirst => "LV_PART_CUSTOM_FIRST".to_string(),
            LVPart::Any => "LV_PART_ANY".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_border_side_serde() {
        for variant in LVPart::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVPart = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_border_side_aliases() {
        let aliases = vec![
            ("main", LVPart::Main),
            ("scrollbar", LVPart::Scrollbar),
            ("indicator", LVPart::Indicator),
            ("knob", LVPart::Knob),
            ("selected", LVPart::Selected),
            ("cursor", LVPart::Cursor),
            ("custom_first", LVPart::CustomFirst),
            ("any", LVPart::Any),
        ];

        assert_eq!(aliases.len(), LVPart::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVPart = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_border_side_invalid() {
        let result: Result<LVPart, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
