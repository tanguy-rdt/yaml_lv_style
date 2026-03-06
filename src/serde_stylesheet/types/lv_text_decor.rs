use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVTextDecor {
    #[serde(rename = "LV_TEXT_DECOR_NONE", alias = "none")]
    None,
    #[serde(rename = "LV_TEXT_DECOR_UNDERLINE", alias = "underline")]
    Underline,
    #[serde(rename = "LV_TEXT_DECOR_STRIKETHROUGH", alias = "strikethrough")]
    Strikethrough,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_text_decor_serde() {
        for variant in LVTextDecor::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVTextDecor = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_text_decor_aliases() {
        let aliases = vec![
            ("none", LVTextDecor::None),
            ("underline", LVTextDecor::Underline),
            ("strikethrough", LVTextDecor::Strikethrough),
        ];

        assert_eq!(aliases.len(), LVTextDecor::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVTextDecor = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_text_decor_invalid() {
        let result: Result<LVTextDecor, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
