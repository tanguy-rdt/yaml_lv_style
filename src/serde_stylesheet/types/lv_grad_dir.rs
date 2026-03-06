use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVGradDir {
    #[serde(rename = "LV_GRAD_DIR_NONE", alias = "none")]
    None,
    #[serde(rename = "LV_GRAD_DIR_VER", alias = "vertical")]
    Vertical,
    #[serde(rename = "LV_GRAD_DIR_HOR", alias = "horizontal")]
    Horizontal,
    #[serde(rename = "LV_GRAD_DIR_LINEAR", alias = "linear")]
    Linear,
    #[serde(rename = "LV_GRAD_DIR_RADIAL", alias = "radial")]
    Radial,
    #[serde(rename = "LV_GRAD_DIR_CONICAL", alias = "conical")]
    Conical,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_grad_dir_serde() {
        for variant in LVGradDir::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVGradDir = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_grad_dir_aliases() {
        let aliases = vec![
            ("none", LVGradDir::None),
            ("vertical", LVGradDir::Vertical),
            ("horizontal", LVGradDir::Horizontal),
            ("linear", LVGradDir::Linear),
            ("radial", LVGradDir::Radial),
            ("conical", LVGradDir::Conical),
        ];

        assert_eq!(aliases.len(), LVGradDir::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVGradDir = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_grad_dir_invalid() {
        let result: Result<LVGradDir, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
