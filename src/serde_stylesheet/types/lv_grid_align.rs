use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVGridAlign {
    #[serde(rename = "LV_GRID_ALIGN_START", alias = "start")]
    Start,
    #[serde(rename = "LV_GRID_ALIGN_CENTER", alias = "center")]
    Center,
    #[serde(rename = "LV_GRID_ALIGN_END", alias = "end")]
    End,
    #[serde(rename = "LV_GRID_ALIGN_STRETCH", alias = "stretch")]
    Stretch,
    #[serde(rename = "LV_GRID_ALIGN_SPACE_EVENLY", alias = "space_evenly")]
    SpaceEvenly,
    #[serde(rename = "LV_GRID_ALIGN_SPACE_AROUND", alias = "space_around")]
    SpaceAround,
    #[serde(rename = "LV_GRID_ALIGN_SPACE_BETWEEN", alias = "space_between")]
    SpaceBetween,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_grid_align_serde() {
        for variant in LVGridAlign::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVGridAlign = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_grid_align_aliases() {
        let aliases = vec![
            ("start", LVGridAlign::Start),
            ("center", LVGridAlign::Center),
            ("end", LVGridAlign::End),
            ("stretch", LVGridAlign::Stretch),
            ("space_evenly", LVGridAlign::SpaceEvenly),
            ("space_around", LVGridAlign::SpaceAround),
            ("space_between", LVGridAlign::SpaceBetween),
        ];

        assert_eq!(aliases.len(), LVGridAlign::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVGridAlign = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_grid_align_invalid() {
        let result: Result<LVGridAlign, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
