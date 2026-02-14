use serde::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

#[cfg_attr(test, derive(PartialEq, strum_macros::EnumIter))]
#[derive(Debug)]
pub enum LVFlexAlign {
    LVFlexAlignStart,
    LVFlexAlignEnd,
    LVFlexAlignCenter,
    LVFlexAlignSpaceEvenly,
    LVFlexAlignSpaceAround,
    LVFlexAlignSpaceBetween,
}

impl<'de> Deserialize<'de> for LVFlexAlign {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "start" | "LV_FLEX_ALIGN_START" => Ok(Self::LVFlexAlignStart),
            "end" | "LV_FLEX_ALIGN_END" => Ok(Self::LVFlexAlignEnd),
            "center" | "LV_FLEX_ALIGN_CENTER" => Ok(Self::LVFlexAlignCenter),
            "space_evenly" | "LV_FLEX_ALIGN_SPACE_EVENLY" => Ok(Self::LVFlexAlignSpaceEvenly),
            "space_around" | "LV_FLEX_ALIGN_SPACE_AROUND" => Ok(Self::LVFlexAlignSpaceAround),
            "space_between" | "LV_FLEX_ALIGN_SPACE_BETWEEN" => Ok(Self::LVFlexAlignSpaceBetween),
            other => Err(serde::de::Error::custom(format!("invalid flex align: {}", other))),
        }
    }
}

impl Serialize for LVFlexAlign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LVFlexAlignStart => "LV_FLEX_ALIGN_START",
            Self::LVFlexAlignEnd => "LV_FLEX_ALIGN_END",
            Self::LVFlexAlignCenter => "LV_FLEX_ALIGN_CENTER",
            Self::LVFlexAlignSpaceEvenly => "LV_FLEX_ALIGN_SPACE_EVENLY",
            Self::LVFlexAlignSpaceAround => "LV_FLEX_ALIGN_SPACE_AROUND",
            Self::LVFlexAlignSpaceBetween => "LV_FLEX_ALIGN_SPACE_BETWEEN",
        };
        serializer.serialize_str(s)
    }
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
            assert_eq!(variant, parsed, "Comparison between serialisation and deserialisation failed for {:?}", variant);
        }
    }
}