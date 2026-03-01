use crate::serde_stylesheet::lv_types::LVColor;
use serde::Deserialize;
use serde::{Serialize, Serializer};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub struct LVImageColorkey {
    pub low: LVColor,
    pub high: LVColor,
}

impl LVImageColorkey {
    pub fn make_const(&mut self) {
        self.low.make_const();
        self.high.make_const();
    }
}

impl Serialize for LVImageColorkey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!(
            "(&(const lv_image_colorkey_t){{{}, {}}})",
            self.low.to_lvgl(),
            self.high.to_lvgl()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lv_image_colorkey_serde() {
        let yaml = r#"
low: hex(#ffffff)
high: rgb(255, 255, 255)
"#;

        let mut parsed: LVImageColorkey = yaml_serde::from_str(yaml).unwrap();
        let out = yaml_serde::to_string(&parsed).unwrap();
        let out = out.trim_end();
        assert_eq!(
            out,
            "(&(const lv_image_colorkey_t){lv_color_make(255, 255, 255), lv_color_make(255, 255, 255)})"
        );

        parsed.make_const();
        let out = yaml_serde::to_string(&parsed).unwrap();
        let out = out.trim_end();
        assert_eq!(
            out,
            "(&(const lv_image_colorkey_t){LV_COLOR_MAKE(255, 255, 255), LV_COLOR_MAKE(255, 255, 255)})"
        );
    }
}
