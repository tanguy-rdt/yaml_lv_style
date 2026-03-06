use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVBaseDir {
    #[serde(rename = "LV_BASE_DIR_LTR", alias = "ltr")]
    Ltr,
    #[serde(rename = "LV_BASE_DIR_RTL", alias = "rtl")]
    Rtl,
    #[serde(rename = "LV_BASE_DIR_AUTO", alias = "auto")]
    Auto,
    #[serde(rename = "LV_BASE_DIR_NEUTRAL", alias = "neutral")]
    Neutral,
    #[serde(rename = "LV_BASE_DIR_WEAK", alias = "weak")]
    Weak,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_base_dir_serde() {
        for variant in LVBaseDir::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVBaseDir = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_base_dir_aliases() {
        let aliases = vec![
            ("ltr", LVBaseDir::Ltr),
            ("rtl", LVBaseDir::Rtl),
            ("auto", LVBaseDir::Auto),
            ("neutral", LVBaseDir::Neutral),
            ("weak", LVBaseDir::Weak),
        ];

        assert_eq!(aliases.len(), LVBaseDir::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVBaseDir = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_base_dir_invalid() {
        let result: Result<LVBaseDir, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
