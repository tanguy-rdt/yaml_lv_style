use std::sync::OnceLock;

use regex::Regex;
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

static HEX_RE: OnceLock<Regex> = OnceLock::new();
static RGB_RE: OnceLock<Regex> = OnceLock::new();

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct LVColor {
    color: LVColorValue,
    serialize_as_const: bool,
}

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
pub enum LVColorValue {
    Hex(u32),
    Rgb(u8, u8, u8),
}

impl LVColor {
    pub fn serialize_as_const(&mut self, state: bool) {
        self.serialize_as_const = state;
    }

    pub fn to_lvgl(&self) -> String {
        let func = if self.serialize_as_const {
            "LV_COLOR_MAKE"
        } else {
            "lv_color_make"
        };

        match self.color {
            LVColorValue::Hex(val) => {
                let r = ((val >> 16) & 0xFFu32) as u8;
                let g = ((val >> 8) & 0xFFu32) as u8;
                let b = (val & 0xFFu32) as u8;
                format!("{}({}, {}, {})", func, r, g, b)
            }
            LVColorValue::Rgb(r, g, b) => {
                format!("{}({}, {}, {})", func, r, g, b)
            }
        }
    }
}

impl<'de> Deserialize<'de> for LVColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.trim();

        let hex_re = HEX_RE.get_or_init(|| {
            Regex::new(r"^(?:0x|#)([0-9a-fA-F]{6})$").expect("Failed to compile HEX_RE regex")
        });

        if let Some(caps) = hex_re.captures(s) {
            let val = u32::from_str_radix(&caps[1], 16)
                .map_err(|_| DeError::custom(format!("invalid hex color: {}", s)))?;
            return Ok(LVColor {
                color: LVColorValue::Hex(val),
                serialize_as_const: false,
            });
        }

        let rgb_re = RGB_RE.get_or_init(|| {
            Regex::new(r"^rgb\(\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*\)$")
                .expect("Failed to compile RGB_RE regex")
        });

        if let Some(caps) = rgb_re.captures(s) {
            let r = caps[1]
                .parse()
                .map_err(|_| DeError::custom("invalid r in RGB color"))?;
            let g = caps[2]
                .parse()
                .map_err(|_| DeError::custom("invalid g in RGB color"))?;
            let b = caps[3]
                .parse()
                .map_err(|_| DeError::custom("invalid b in RGB color"))?;
            return Ok(LVColor {
                color: LVColorValue::Rgb(r, g, b),
                serialize_as_const: false,
            });
        }

        Err(DeError::custom(format!(
            "invalid color format: '{}', use '#FF0000', '0xFF0000' or 'rgb(255, 0, 0)'",
            s
        )))
    }
}

impl Serialize for LVColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_lvgl().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lv_color_deserialize() {
        let rgb: LVColor = yaml_serde::from_str("rgb(10, 20, 30)").unwrap();
        match rgb.color {
            LVColorValue::Rgb(r, g, b) => {
                assert_eq!(r, 10);
                assert_eq!(g, 20);
                assert_eq!(b, 30);
            }
            _ => panic!("expected Rgb"),
        }

        let hex_hash: LVColor = yaml_serde::from_str("\"#FF00FF\"").unwrap();
        match hex_hash.color {
            LVColorValue::Hex(v) => assert_eq!(v, 0xFF00FF),
            _ => panic!("expected Hex"),
        }

        let hex_0x: LVColor = yaml_serde::from_str("0xFF00FF").unwrap();
        match hex_0x.color {
            LVColorValue::Hex(v) => assert_eq!(v, 0xFF00FF),
            _ => panic!("expected Hex"),
        }
    }

    #[test]
    fn test_lv_color_serialize() {
        let mut rgb = LVColor {
            color: LVColorValue::Rgb(0, 1, 255),
            serialize_as_const: false,
        };
        let out_rgb = yaml_serde::to_string(&rgb).unwrap();
        assert_eq!(out_rgb.trim(), "lv_color_make(0, 1, 255)");

        rgb.serialize_as_const = true;
        let out_rgb = yaml_serde::to_string(&rgb).unwrap();
        assert_eq!(out_rgb.trim(), "LV_COLOR_MAKE(0, 1, 255)");

        let mut hex = LVColor {
            color: LVColorValue::Hex(0x0001ff),
            serialize_as_const: false,
        };
        let out_hex = yaml_serde::to_string(&hex).unwrap();
        assert_eq!(out_hex.trim(), "lv_color_make(0, 1, 255)");

        hex.serialize_as_const = true;
        let out_hex = yaml_serde::to_string(&hex).unwrap();
        assert_eq!(out_hex.trim(), "LV_COLOR_MAKE(0, 1, 255)");
    }

    #[test]
    fn test_lv_color_deserialization_invalid_value() {
        let bad: Result<LVColor, _> = yaml_serde::from_str("blue");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("rgb(-1, 0, 0)");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("rgb(256, 0, 0)");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("hex(FF0000)");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("#GG0000");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("#FF000");
        assert!(bad.is_err());
    }
}
