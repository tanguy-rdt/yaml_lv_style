use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(PartialEq, strum_macros::EnumIter))]
#[derive(Debug, Deserialize, Serialize)]
pub enum LVBlendMode {
    #[serde(rename = "LV_BLEND_MODE_NORMAL", alias = "none")]
    Normal,
    #[serde(rename = "LV_BLEND_MODE_ADDITIVE", alias = "additive")]
    Additive,
    #[serde(rename = "LV_BLEND_MODE_SUBTRACTIVE", alias = "subtractive")]
    Subtractive,
    #[serde(rename = "LV_BLEND_MODE_MULTIPLY", alias = "multiply")]
    Multiply,
    #[serde(rename = "LV_BLEND_MODE_DIFFERENCE", alias = "difference")]
    Difference,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_blend_mode_serde() {
        for variant in LVBlendMode::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVBlendMode = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(
                variant, parsed,
                "Comparison between serialisation and deserialisation failed for {:?}",
                variant
            );
        }
    }

    #[test]
    fn test_lv_blend_mode_aliases() {
        let aliases = vec![
            ("none", LVBlendMode::Normal),
            ("additive", LVBlendMode::Additive),
            ("subtractive", LVBlendMode::Subtractive),
            ("multiply", LVBlendMode::Multiply),
            ("difference", LVBlendMode::Difference),
        ];

        assert_eq!(aliases.len(), LVBlendMode::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVBlendMode = yaml_serde::from_str(alias).unwrap();
            assert_eq!(
                parsed, expected_variant,
                "The alias ‘{}’ was not correctly deserialized to {:?}",
                alias, expected_variant
            );
        }
    }
}
