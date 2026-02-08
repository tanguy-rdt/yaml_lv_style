use serde::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[derive(Debug, PartialEq)]
pub enum LVTextAlign {
    LvTextAlignAuto,
    LvTextAlignLeft,
    LvTextAlignCenter,
    LvTextAlignRight
}

impl<'de> Deserialize<'de> for LVTextAlign {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "auto" | "LV_TEXT_ALIGN_AUTO" => Ok(Self::LvTextAlignAuto),
            "left" | "LV_TEXT_ALIGN_LEFT" => Ok(Self::LvTextAlignLeft),
            "center" | "LV_TEXT_ALIGN_CENTER" => Ok(Self::LvTextAlignCenter),
            "right" | "LV_TEXT_ALIGN_RIGHT" => Ok(Self::LvTextAlignRight),
            other => Err(serde::de::Error::custom(format!("invalid text align: {}", other))),
        }
    }
}

impl Serialize for LVTextAlign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LvTextAlignAuto => "LV_TEXT_ALIGN_AUTO",
            Self::LvTextAlignLeft => "LV_TEXT_ALIGN_LEFT",
            Self::LvTextAlignCenter => "LV_TEXT_ALIGN_CENTER",
            Self::LvTextAlignRight => "LV_TEXT_ALIGN_RIGHT",
        };
        serializer.serialize_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_text_align_serde() {
        for variant in LVTextAlign::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVTextAlign = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed, "Comparison between serialisation and deserialisation failed for {:?}", variant);
        }
    }
}