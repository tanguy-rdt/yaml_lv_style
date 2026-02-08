use serde::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[derive(Debug, PartialEq)]
pub enum LVBorderSide {
    LVBorderSideNone,
    LVBorderSideBottom,
    LVBorderSideTop,
    LVBorderSideLeft,
    LVBorderSideRight,
    LVBorderSideFull,
    LVBorderSideInternal,
}

impl<'de> Deserialize<'de> for LVBorderSide {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "none" | "LV_BORDER_SIDE_NONE" => Ok(Self::LVBorderSideNone),
            "bottom" | "LV_BORDER_SIDE_BOTTOM" => Ok(Self::LVBorderSideBottom),
            "top" | "LV_BORDER_SIDE_TOP" => Ok(Self::LVBorderSideTop),
            "left" | "LV_BORDER_SIDE_LEFT" => Ok(Self::LVBorderSideLeft),
            "right" | "LV_BORDER_SIDE_RIGHT" => Ok(Self::LVBorderSideRight),
            "full" | "LV_BORDER_SIDE_FULL" => Ok(Self::LVBorderSideFull),
            "internal" | "LV_BORDER_SIDE_INTERNAL" => Ok(Self::LVBorderSideInternal),
            other => Err(serde::de::Error::custom(format!("invalid border side: {}", other))),
        }
    }
}

impl Serialize for LVBorderSide {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LVBorderSideNone => "LV_BORDER_SIDE_NONE",
            Self::LVBorderSideBottom => "LV_BORDER_SIDE_BOTTOM",
            Self::LVBorderSideTop => "LV_BORDER_SIDE_TOP",
            Self::LVBorderSideLeft => "LV_BORDER_SIDE_LEFT",
            Self::LVBorderSideRight => "LV_BORDER_SIDE_RIGHT",
            Self::LVBorderSideFull => "LV_BORDER_SIDE_FULL",
            Self::LVBorderSideInternal => "LV_BORDER_SIDE_INTERNAL",
        };
        serializer.serialize_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_border_side_serde() {
        for variant in LVBorderSide::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVBorderSide = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed, "Comparison between serialisation and deserialisation failed for {:?}", variant);
        }
    }
}