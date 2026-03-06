use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVBorderSide {
    #[serde(rename = "LV_BORDER_SIDE_NONE", alias = "none")]
    None,
    #[serde(rename = "LV_BORDER_SIDE_BOTTOM", alias = "bottom")]
    Bottom,
    #[serde(rename = "LV_BORDER_SIDE_TOP", alias = "top")]
    Top,
    #[serde(rename = "LV_BORDER_SIDE_LEFT", alias = "left")]
    Left,
    #[serde(rename = "LV_BORDER_SIDE_RIGHT", alias = "right")]
    Right,
    #[serde(rename = "LV_BORDER_SIDE_FULL", alias = "full")]
    Full,
    #[serde(rename = "LV_BORDER_SIDE_INTERNAL", alias = "internal")]
    Internal,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_border_side_serde() {
        for variant in LVBorderSide::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVBorderSide = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_border_side_aliases() {
        let aliases = vec![
            ("none", LVBorderSide::None),
            ("bottom", LVBorderSide::Bottom),
            ("top", LVBorderSide::Top),
            ("left", LVBorderSide::Left),
            ("right", LVBorderSide::Right),
            ("full", LVBorderSide::Full),
            ("internal", LVBorderSide::Internal),
        ];

        assert_eq!(aliases.len(), LVBorderSide::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVBorderSide = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_border_side_invalid() {
        let result: Result<LVBorderSide, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
