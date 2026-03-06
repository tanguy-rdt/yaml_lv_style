use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVOpa {
    #[serde(rename = "LV_OPA_TRANSP", alias = "transparent")]
    Transparent,
    #[serde(rename = "LV_OPA_0", alias = "opa_0")]
    Opa0,
    #[serde(rename = "LV_OPA_10", alias = "opa_10")]
    Opa10,
    #[serde(rename = "LV_OPA_20", alias = "opa_20")]
    Opa20,
    #[serde(rename = "LV_OPA_30", alias = "opa_30")]
    Opa30,
    #[serde(rename = "LV_OPA_40", alias = "opa_40")]
    Opa40,
    #[serde(rename = "LV_OPA_50", alias = "opa_50")]
    Opa50,
    #[serde(rename = "LV_OPA_60", alias = "opa_60")]
    Opa60,
    #[serde(rename = "LV_OPA_70", alias = "opa_70")]
    Opa70,
    #[serde(rename = "LV_OPA_80", alias = "opa_80")]
    Opa80,
    #[serde(rename = "LV_OPA_90", alias = "opa_90")]
    Opa90,
    #[serde(rename = "LV_OPA_100", alias = "opa_100")]
    Opa100,
    #[serde(rename = "LV_OPA_COVER", alias = "cover")]
    Cover,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_opa_serde() {
        for variant in LVOpa::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVOpa = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_opa_aliases() {
        let aliases = vec![
            ("transparent", LVOpa::Transparent),
            ("opa_0", LVOpa::Opa0),
            ("opa_10", LVOpa::Opa10),
            ("opa_20", LVOpa::Opa20),
            ("opa_30", LVOpa::Opa30),
            ("opa_40", LVOpa::Opa40),
            ("opa_50", LVOpa::Opa50),
            ("opa_60", LVOpa::Opa60),
            ("opa_70", LVOpa::Opa70),
            ("opa_80", LVOpa::Opa80),
            ("opa_90", LVOpa::Opa90),
            ("opa_100", LVOpa::Opa100),
            ("cover", LVOpa::Cover),
        ];

        assert_eq!(aliases.len(), LVOpa::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVOpa = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_opa_invalid() {
        let result: Result<LVOpa, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
