use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

#[cfg_attr(test, derive(PartialEq, strum_macros::EnumIter))]
#[derive(Debug)]
pub enum LVGridAlign {
    LVGridAlignStart,
    LVGridAlignCenter,
    LVGridAlignEnd,
    LVGridAlignStretch,
    LVGridAlignSpaceEvenly,
    LVGridAlignSpaceAround,
    LVGridAlignSpaceBetween,
}

impl<'de> Deserialize<'de> for LVGridAlign {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "start" | "LV_GRID_ALIGN_START" => Ok(Self::LVGridAlignStart),
            "center" | "LV_GRID_ALIGN_CENTER" => Ok(Self::LVGridAlignCenter),
            "end" | "LV_GRID_ALIGN_END" => Ok(Self::LVGridAlignEnd),
            "stretch" | "LV_GRID_ALIGN_STRETCH" => Ok(Self::LVGridAlignStretch),
            "space_evenly" | "LV_GRID_ALIGN_SPACE_EVENLY" => Ok(Self::LVGridAlignSpaceEvenly),
            "space_around" | "LV_GRID_ALIGN_SPACE_AROUND" => Ok(Self::LVGridAlignSpaceAround),
            "space_between" | "LV_GRID_ALIGN_SPACE_BETWEEN" => Ok(Self::LVGridAlignSpaceBetween),
            other => Err(serde::de::Error::custom(format!(
                "invalid grid align: {}",
                other
            ))),
        }
    }
}

impl Serialize for LVGridAlign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LVGridAlignStart => "LV_GRID_ALIGN_START",
            Self::LVGridAlignCenter => "LV_GRID_ALIGN_CENTER",
            Self::LVGridAlignEnd => "LV_GRID_ALIGN_END",
            Self::LVGridAlignStretch => "LV_GRID_ALIGN_STRETCH",
            Self::LVGridAlignSpaceEvenly => "LV_GRID_ALIGN_SPACE_EVENLY",
            Self::LVGridAlignSpaceAround => "LV_GRID_ALIGN_SPACE_AROUND",
            Self::LVGridAlignSpaceBetween => "LV_GRID_ALIGN_SPACE_BETWEEN",
        };
        serializer.serialize_str(s)
    }
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
            assert_eq!(
                variant, parsed,
                "Comparison between serialisation and deserialisation {:?}",
                variant
            );
        }
    }
}
