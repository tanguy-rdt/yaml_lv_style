use serde::{Deserialize, Serialize};

use super::lv_properties::LVProperties;

#[derive(Debug, Default, PartialEq)]
#[derive(Deserialize, Serialize)]
pub struct Style {
    pub name: Option<String>,
    #[serde(alias = "const")]
    pub const_style: Option<bool>,
    pub default: Option<LVProperties>,
    pub checked: Option<LVProperties>,
    pub focused: Option<LVProperties>,
    pub focus_key: Option<LVProperties>,
    pub edited: Option<LVProperties>,
    pub hovered: Option<LVProperties>,
    pub pressed: Option<LVProperties>,
    pub disabled: Option<LVProperties>,
    pub user_1: Option<LVProperties>,
    pub user_2: Option<LVProperties>,
    pub user_3: Option<LVProperties>,
    pub user_4: Option<LVProperties>,
    pub any: Option<LVProperties>,
}

#[cfg(test)]
mod tests {
    use yaml_serde;

    use super::*;

    use crate::stylesheet::lv_types::lv_align::LVAlign;
    use crate::stylesheet::lv_types::lv_color::LVColor;

    #[test]
    fn test_style_serde() {
        let props_default = LVProperties {
            width: Some(100),
            bg_color: Some(LVColor::Rgb(212, 212, 212)),
            border_color: Some(LVColor::Rgb(191, 191, 191)),
            align: Some(LVAlign::LvAlignCenter),
            ..Default::default()
        };

        let props_hovered = LVProperties {
            border_color: Some(LVColor::Rgb(209, 100, 63)),
            ..Default::default()
        };

        let style = Style {
            name: Some("test_style".to_string()),
            const_style: Some(true),
            default: Some(props_default),
            checked: Some(props_hovered),
            ..Default::default()
        };

        let yaml = yaml_serde::to_string(&style).unwrap();
        let parsed: Style = yaml_serde::from_str(&yaml).unwrap();

        assert_eq!(style.name, parsed.name);
        assert_eq!(style.const_style, parsed.const_style);
        assert_eq!(style.default, parsed.default);
        assert_eq!(style.checked, parsed.checked);
    }
}