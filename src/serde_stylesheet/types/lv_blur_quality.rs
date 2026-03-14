use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVBlurQuality {
    #[serde(rename = "LV_BLUR_QUALITY_AUTO", alias = "auto")]
    Auto,
    #[serde(rename = "LV_BLUR_QUALITY_SPEED", alias = "speed")]
    Speed,
    #[serde(rename = "LV_BLUR_QUALITY_PRECISION", alias = "precision")]
    Precision,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_blur_quality_serde() {
        for variant in LVBlurQuality::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVBlurQuality = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_blur_quality_aliases() {
        let aliases = vec![
            ("auto", LVBlurQuality::Auto),
            ("speed", LVBlurQuality::Speed),
            ("precision", LVBlurQuality::Precision),
        ];

        assert_eq!(aliases.len(), LVBlurQuality::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVBlurQuality = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_blur_quality_invalid() {
        let result: Result<LVBlurQuality, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
