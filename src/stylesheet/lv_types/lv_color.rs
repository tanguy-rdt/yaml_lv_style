use serde::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};
use serde::de::{Error as DeError};
use regex::Regex;

#[cfg_attr(test, derive(PartialEq, strum_macros::EnumIter))]
#[derive(Debug)]
pub enum LVColor {
    Hex(u32),
    Rgb(u8, u8, u8),
}

impl<'de> Deserialize<'de> for LVColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.trim();

        let hex_re = Regex::new(r"^(?:hex|lv_color_hex)\(\s*(?:0x|#)?([0-9a-fA-F]{6})\s*\)$").unwrap();
        if let Some(caps) = hex_re.captures(s) {
            let val = u32::from_str_radix(&caps[1], 16)
                .map_err(|_| DeError::custom(format!("Hex invalide: {}", s)))?;
            return Ok(Self::Hex(val));
        }

        let rgb_re = Regex::new(r"^(?:rgb|lv_color_make)\(\s*([0-9]+)\s*,\s*([0-9]+)\s*,\s*([0-9]+)\s*\)$").unwrap();
        if let Some(caps) = rgb_re.captures(s) {
            let r = caps[1].parse().map_err(|_| DeError::custom("invalid r in RGB color"))?;
            let g = caps[2].parse().map_err(|_| DeError::custom("invalid g in RGB color"))?;
            let b = caps[3].parse().map_err(|_| DeError::custom("invalid b in RGB color"))?;
            return Ok(Self::Rgb(r, g, b));
        }

        Err(DeError::custom(format!("invalid color format: {}, use hex(000000) or rgb(0, 0, 0)", s)))
    }
}

impl Serialize for LVColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Hex(val) => {
                let r = ((val >> 16) & 0xFFu32) as u8;
                let g = ((val >> 8) & 0xFFu32) as u8;
                let b = (val & 0xFFu32) as u8;
                serializer.serialize_str(&format!("lv_color_make({}, {}, {})", r, g, b))
            },
            Self::Rgb(r,g,b) => serializer.serialize_str(&format!("lv_color_make({}, {}, {})", r,g,b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lv_color_deserialize() {
        let rgb: LVColor = yaml_serde::from_str("rgb(10, 20, 30)").unwrap();
        match rgb {
            LVColor::Rgb(r, g, b) => {
                assert_eq!(r, 10);
                assert_eq!(g, 20);
                assert_eq!(b, 30);
            }
            _ => panic!("expected Rgb"),
        }

        let hex: LVColor = yaml_serde::from_str("hex(0xFF00FF)").unwrap();
        match hex {
            LVColor::Hex(v) => assert_eq!(v, 0xFF00FF),
            _ => panic!("expected Hex"),
        }
    }


    #[test]
    fn test_lv_color_serialize() {
        let rgb = LVColor::Rgb(0, 1, 255);
        let out_rgb = yaml_serde::to_string(&rgb).unwrap();
        assert_eq!(out_rgb.trim(), "lv_color_make(0, 1, 255)");

        let hex = LVColor::Hex(0x0001ff);
        let out_hex = yaml_serde::to_string(&hex).unwrap();
        assert_eq!(out_hex.trim(), "lv_color_make(0, 1, 255)");
    }

    #[test]
    fn test_lv_color_invalid_input() {
        let bad: Result<LVColor, _> = yaml_serde::from_str("blue");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("rgb(-1, 0, 0)");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("rgb(256, 0, 0)");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("hex(000000f)");
        assert!(bad.is_err());

        let bad: Result<LVColor, _> = yaml_serde::from_str("hex(00000g)");
        assert!(bad.is_err());
    }
}