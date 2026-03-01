use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVTextAlign {
    #[serde(rename = "LV_TEXT_ALIGN_AUTO", alias = "auto")]
    Auto,
    #[serde(rename = "LV_TEXT_ALIGN_LEFT", alias = "left")]
    Left,
    #[serde(rename = "LV_TEXT_ALIGN_CENTER", alias = "center")]
    Center,
    #[serde(rename = "LV_TEXT_ALIGN_RIGHT", alias = "right")]
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_text_align_serde() {
        for variant in LVTextAlign::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVTextAlign = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(
                variant, parsed,
                "Comparison between serialisation and deserialisation failed for {:?}",
                variant
            );
        }
    }

    #[test]
    fn test_lv_text_align_aliases() {
        let aliases = vec![
            ("auto", LVTextAlign::Auto),
            ("left", LVTextAlign::Left),
            ("center", LVTextAlign::Center),
            ("right", LVTextAlign::Right),
        ];

        assert_eq!(aliases.len(), LVTextAlign::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVTextAlign = yaml_serde::from_str(alias).unwrap();
            assert_eq!(
                parsed, expected_variant,
                "The alias ‘{}’ was not correctly deserialized to {:?}",
                alias, expected_variant
            );
        }
    }
}
