use serde::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

#[cfg_attr(test, derive(PartialEq, strum_macros::EnumIter))]
#[derive(Debug)]
pub enum LVAlign {
    LvAlignDefault,
    LvAlignTopLeft,
    LvAlignTopMid,
    LvAlignTopRight,
    LvAlignBottomLeft,
    LvAlignBottomMid,
    LvAlignBottomRight,
    LvAlignLeftMid,
    LvAlignRightMid,
    LvAlignCenter,
    LvAlignOutTopLeft,
    LvAlignOutTopMid,
    LvAlignOutTopRight,
    LvAlignOutBottomLeft,
    LvAlignOutBottomMid,
    LvAlignOutBottomRight,
    LvAlignOutLeftTop,
    LvAlignOutLeftMid,
    LvAlignOutLeftBottom,
    LvAlignOutRightTop,
    LvAlignOutRightMid,
    LvAlignOutRightBottom,
}

impl<'de> Deserialize<'de> for LVAlign {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "default" | "LV_ALIGN_DEFAULT" => Ok(Self::LvAlignDefault),
            "top_left" | "LV_ALIGN_TOP_LEFT" => Ok(Self::LvAlignTopLeft),
            "top_mid" | "LV_ALIGN_TOP_MID" => Ok(Self::LvAlignTopMid),
            "top_right" | "LV_ALIGN_TOP_RIGHT" => Ok(Self::LvAlignTopRight),
            "bottom_left" | "LV_ALIGN_BOTTOM_LEFT" => Ok(Self::LvAlignBottomLeft),
            "bottom_mid" | "LV_ALIGN_BOTTOM_MID" => Ok(Self::LvAlignBottomMid),
            "bottom_right" | "LV_ALIGN_BOTTOM_RIGHT" => Ok(Self::LvAlignBottomRight),
            "left_mid" | "LV_ALIGN_LEFT_MID" => Ok(Self::LvAlignLeftMid),
            "right_mid" | "LV_ALIGN_RIGHT_MID" => Ok(Self::LvAlignRightMid),
            "center" | "LV_ALIGN_CENTER" => Ok(Self::LvAlignCenter),
            "out_top_left" | "LV_ALIGN_OUT_TOP_LEFT" => Ok(Self::LvAlignOutTopLeft),
            "out_top_mid" | "LV_ALIGN_OUT_TOP_MID" => Ok(Self::LvAlignOutTopMid),
            "out_top_right" | "LV_ALIGN_OUT_TOP_RIGHT" => Ok(Self::LvAlignOutTopRight),
            "out_bottom_left" | "LV_ALIGN_OUT_BOTTOM_LEFT" => Ok(Self::LvAlignOutBottomLeft),
            "out_bottom_mid" | "LV_ALIGN_OUT_BOTTOM_MID" => Ok(Self::LvAlignOutBottomMid),
            "out_bottom_right" | "LV_ALIGN_OUT_BOTTOM_RIGHT" => Ok(Self::LvAlignOutBottomRight),
            "out_left_top" | "LV_ALIGN_OUT_LEFT_TOP" => Ok(Self::LvAlignOutLeftTop),
            "out_left_mid" | "LV_ALIGN_OUT_LEFT_MID" => Ok(Self::LvAlignOutLeftMid),
            "out_left_bottom" | "LV_ALIGN_OUT_LEFT_BOTTOM" => Ok(Self::LvAlignOutLeftBottom),
            "out_right_top" | "LV_ALIGN_OUT_RIGHT_TOP" => Ok(Self::LvAlignOutRightTop),
            "out_right_mid" | "LV_ALIGN_OUT_RIGHT_MID" => Ok(Self::LvAlignOutRightMid),
            "out_right_bottom" | "LV_ALIGN_OUT_RIGHT_BOTTOM" => Ok(Self::LvAlignOutRightBottom),
            other => Err(serde::de::Error::custom(format!("invalid alignment: {}", other))),
        }
    }
}

impl Serialize for LVAlign {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LvAlignDefault => "LV_ALIGN_DEFAULT",
            Self::LvAlignTopLeft => "LV_ALIGN_TOP_LEFT",
            Self::LvAlignTopMid => "LV_ALIGN_TOP_MID",
            Self::LvAlignTopRight => "LV_ALIGN_TOP_RIGHT",
            Self::LvAlignBottomLeft => "LV_ALIGN_BOTTOM_LEFT",
            Self::LvAlignBottomMid => "LV_ALIGN_BOTTOM_MID",
            Self::LvAlignBottomRight => "LV_ALIGN_BOTTOM_RIGHT",
            Self::LvAlignLeftMid => "LV_ALIGN_LEFT_MID",
            Self::LvAlignRightMid => "LV_ALIGN_RIGHT_MID",
            Self::LvAlignCenter => "LV_ALIGN_CENTER",
            Self::LvAlignOutTopLeft => "LV_ALIGN_OUT_TOP_LEFT",
            Self::LvAlignOutTopMid => "LV_ALIGN_OUT_TOP_MID",
            Self::LvAlignOutTopRight => "LV_ALIGN_OUT_TOP_RIGHT",
            Self::LvAlignOutBottomLeft => "LV_ALIGN_OUT_BOTTOM_LEFT",
            Self::LvAlignOutBottomMid => "LV_ALIGN_OUT_BOTTOM_MID",
            Self::LvAlignOutBottomRight => "LV_ALIGN_OUT_BOTTOM_RIGHT",
            Self::LvAlignOutLeftTop => "LV_ALIGN_OUT_LEFT_TOP",
            Self::LvAlignOutLeftMid => "LV_ALIGN_OUT_LEFT_MID",
            Self::LvAlignOutLeftBottom => "LV_ALIGN_OUT_LEFT_BOTTOM",
            Self::LvAlignOutRightTop => "LV_ALIGN_OUT_RIGHT_TOP",
            Self::LvAlignOutRightMid => "LV_ALIGN_OUT_RIGHT_MID",
            Self::LvAlignOutRightBottom => "LV_ALIGN_OUT_RIGHT_BOTTOM"
        };
        serializer.serialize_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_align_serde() {
        for variant in LVAlign::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVAlign = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed, "Comparison between serialisation and deserialisation failed for {:?}", variant);
        }
    }
}