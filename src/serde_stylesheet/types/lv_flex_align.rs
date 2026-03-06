use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVFlexAlign {
    #[serde(rename = "LV_FLEX_ALIGN_START", alias = "start")]
    Start,
    #[serde(rename = "LV_FLEX_ALIGN_END", alias = "end")]
    End,
    #[serde(rename = "LV_FLEX_ALIGN_CENTER", alias = "center")]
    Center,
    #[serde(rename = "LV_FLEX_ALIGN_SPACE_EVENLY", alias = "space_evenly")]
    SpaceEvenly,
    #[serde(rename = "LV_FLEX_ALIGN_SPACE_AROUND", alias = "space_around")]
    SpaceAround,
    #[serde(rename = "LV_FLEX_ALIGN_SPACE_BETWEEN", alias = "space_between")]
    SpaceBetween,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_flex_align_serde() {
        for variant in LVFlexAlign::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVFlexAlign = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_flex_align_aliases() {
        let aliases = vec![
            ("start", LVFlexAlign::Start),
            ("end", LVFlexAlign::End),
            ("center", LVFlexAlign::Center),
            ("space_evenly", LVFlexAlign::SpaceEvenly),
            ("space_around", LVFlexAlign::SpaceAround),
            ("space_between", LVFlexAlign::SpaceBetween),
        ];

        assert_eq!(aliases.len(), LVFlexAlign::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVFlexAlign = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_flex_align_invalid() {
        let result: Result<LVFlexAlign, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
