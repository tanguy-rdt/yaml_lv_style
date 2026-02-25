use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

#[cfg_attr(test, derive(PartialEq, strum_macros::EnumIter))]
#[derive(Debug)]
pub enum LVOpa {
    LVOpaTransp,
    LVOpa0,
    LVOpa10,
    LVOpa20,
    LVOpa30,
    LVOpa40,
    LVOpa50,
    LVOpa60,
    LVOpa70,
    LVOpa80,
    LVOpa90,
    LVOpa100,
    LVOpaCover,
}

impl<'de> Deserialize<'de> for LVOpa {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "transparent" | "LV_OPA_TRANSP" => Ok(Self::LVOpaTransp),
            "opa_0" | "LV_OPA_0" => Ok(Self::LVOpa0),
            "opa_10" | "LV_OPA_10" => Ok(Self::LVOpa10),
            "opa_20" | "LV_OPA_20" => Ok(Self::LVOpa20),
            "opa_30" | "LV_OPA_30" => Ok(Self::LVOpa30),
            "opa_40" | "LV_OPA_40" => Ok(Self::LVOpa40),
            "opa_50" | "LV_OPA_50" => Ok(Self::LVOpa50),
            "opa_60" | "LV_OPA_60" => Ok(Self::LVOpa60),
            "opa_70" | "LV_OPA_70" => Ok(Self::LVOpa70),
            "opa_80" | "LV_OPA_80" => Ok(Self::LVOpa80),
            "opa_90" | "LV_OPA_90" => Ok(Self::LVOpa90),
            "opa_100" | "LV_OPA_100" => Ok(Self::LVOpa100),
            "cover" | "LV_OPA_COVER" => Ok(Self::LVOpaCover),
            other => Err(serde::de::Error::custom(format!(
                "invalid opacity: {}",
                other
            ))),
        }
    }
}

impl Serialize for LVOpa {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LVOpaTransp => "LV_OPA_TRANSP",
            Self::LVOpa0 => "LV_OPA_0",
            Self::LVOpa10 => "LV_OPA_10",
            Self::LVOpa20 => "LV_OPA_20",
            Self::LVOpa30 => "LV_OPA_30",
            Self::LVOpa40 => "LV_OPA_40",
            Self::LVOpa50 => "LV_OPA_50",
            Self::LVOpa60 => "LV_OPA_60",
            Self::LVOpa70 => "LV_OPA_70",
            Self::LVOpa80 => "LV_OPA_80",
            Self::LVOpa90 => "LV_OPA_90",
            Self::LVOpa100 => "LV_OPA_100",
            Self::LVOpaCover => "LV_OPA_COVER",
        };
        serializer.serialize_str(s)
    }
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
            assert_eq!(
                variant, parsed,
                "Comparison between serialisation and deserialisation failed for {:?}",
                variant
            );
        }
    }
}
