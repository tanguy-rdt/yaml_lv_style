use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVAlign {
    #[serde(rename = "LV_ALIGN_DEFAULT", alias = "default")]
    Default,
    #[serde(rename = "LV_ALIGN_TOP_LEFT", alias = "top_left")]
    TopLeft,
    #[serde(rename = "LV_ALIGN_TOP_MID", alias = "top_mid")]
    TopMid,
    #[serde(rename = "LV_ALIGN_TOP_RIGHT", alias = "top_right")]
    TopRight,
    #[serde(rename = "LV_ALIGN_BOTTOM_LEFT", alias = "bottom_left")]
    BottomLeft,
    #[serde(rename = "LV_ALIGN_BOTTOM_MID", alias = "bottom_mid")]
    BottomMid,
    #[serde(rename = "LV_ALIGN_BOTTOM_RIGHT", alias = "bottom_right")]
    BottomRight,
    #[serde(rename = "LV_ALIGN_LEFT_MID", alias = "left_mid")]
    LeftMid,
    #[serde(rename = "LV_ALIGN_RIGHT_MID", alias = "right_mid")]
    RightMid,
    #[serde(rename = "LV_ALIGN_CENTER", alias = "center")]
    Center,
    #[serde(rename = "LV_ALIGN_OUT_TOP_LEFT", alias = "out_top_left")]
    OutTopLeft,
    #[serde(rename = "LV_ALIGN_OUT_TOP_MID", alias = "out_top_mid")]
    OutTopMid,
    #[serde(rename = "LV_ALIGN_OUT_TOP_RIGHT", alias = "out_top_right")]
    OutTopRight,
    #[serde(rename = "LV_ALIGN_OUT_BOTTOM_LEFT", alias = "out_bottom_left")]
    OutBottomLeft,
    #[serde(rename = "LV_ALIGN_OUT_BOTTOM_MID", alias = "out_bottom_mid")]
    OutBottomMid,
    #[serde(rename = "LV_ALIGN_OUT_BOTTOM_RIGHT", alias = "out_bottom_right")]
    OutBottomRight,
    #[serde(rename = "LV_ALIGN_OUT_LEFT_TOP", alias = "out_left_top")]
    OutLeftTop,
    #[serde(rename = "LV_ALIGN_OUT_LEFT_MID", alias = "out_left_mid")]
    OutLeftMid,
    #[serde(rename = "LV_ALIGN_OUT_LEFT_BOTTOM", alias = "out_left_bottom")]
    OutLeftBottom,
    #[serde(rename = "LV_ALIGN_OUT_RIGHT_TOP", alias = "out_right_top")]
    OutRightTop,
    #[serde(rename = "LV_ALIGN_OUT_RIGHT_MID", alias = "out_right_mid")]
    OutRightMid,
    #[serde(rename = "LV_ALIGN_OUT_RIGHT_BOTTOM", alias = "out_right_bottom")]
    OutRightBottom,
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
            assert_eq!(
                variant, parsed,
                "Comparison between serialisation and deserialisation failed for {:?}",
                variant
            );
        }
    }

    #[test]
    fn test_lv_align_aliases() {
        let aliases = vec![
            ("default", LVAlign::Default),
            ("top_left", LVAlign::TopLeft),
            ("top_mid", LVAlign::TopMid),
            ("top_right", LVAlign::TopRight),
            ("bottom_left", LVAlign::BottomLeft),
            ("bottom_mid", LVAlign::BottomMid),
            ("bottom_right", LVAlign::BottomRight),
            ("left_mid", LVAlign::LeftMid),
            ("right_mid", LVAlign::RightMid),
            ("center", LVAlign::Center),
            ("out_top_left", LVAlign::OutTopLeft),
            ("out_top_mid", LVAlign::OutTopMid),
            ("out_top_right", LVAlign::OutTopRight),
            ("out_bottom_left", LVAlign::OutBottomLeft),
            ("out_bottom_mid", LVAlign::OutBottomMid),
            ("out_bottom_right", LVAlign::OutBottomRight),
            ("out_left_top", LVAlign::OutLeftTop),
            ("out_left_mid", LVAlign::OutLeftMid),
            ("out_left_bottom", LVAlign::OutLeftBottom),
            ("out_right_top", LVAlign::OutRightTop),
            ("out_right_mid", LVAlign::OutRightMid),
            ("out_right_bottom", LVAlign::OutRightBottom),
        ];

        assert_eq!(aliases.len(), LVAlign::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVAlign = yaml_serde::from_str(alias).unwrap();
            assert_eq!(
                parsed, expected_variant,
                "The alias ‘{}’ was not correctly deserialized to {:?}",
                alias, expected_variant
            );
        }
    }
}
