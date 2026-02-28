use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(PartialEq, strum_macros::EnumIter))]
#[derive(Debug, Deserialize, Serialize)]
pub enum LVLayout {
    #[serde(rename = "LV_LAYOUT_NONE", alias = "none")]
    None,
    #[serde(rename = "LV_LAYOUT_FLEX", alias = "flex")]
    Flex,
    #[serde(rename = "LV_LAYOUT_GRID", alias = "grid")]
    Grid,
    #[serde(rename = "LV_LAYOUT_LAST", alias = "last")]
    Last,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_text_decor_serde() {
        for variant in LVLayout::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVLayout = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(
                variant, parsed,
                "Comparison between serialisation and deserialisation failed for {:?}",
                variant
            );
        }
    }

    #[test]
    fn test_lv_text_decor_aliases() {
        let aliases = vec![
            ("none", LVLayout::None),
            ("flex", LVLayout::Flex),
            ("grid", LVLayout::Grid),
            ("last", LVLayout::Last),
        ];

        assert_eq!(aliases.len(), LVLayout::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVLayout = yaml_serde::from_str(alias).unwrap();
            assert_eq!(
                parsed, expected_variant,
                "The alias ‘{}’ was not correctly deserialized to {:?}",
                alias, expected_variant
            );
        }
    }
}
