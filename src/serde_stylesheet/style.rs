use serde::{Deserialize, Serialize};

use super::lv_properties::LVProperties;

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

impl Style {
    pub fn add(&mut self, other: Style) {
        macro_rules! merge {
            ($field:ident) => {
                if let Some(val) = other.$field {
                    self.$field = Some(val);
                }
            };
        }

        merge!(const_style);
        merge!(default);
        merge!(checked);
        merge!(focused);
        merge!(focus_key);
        merge!(edited);
        merge!(hovered);
        merge!(pressed);
        merge!(disabled);
        merge!(user_1);
        merge!(user_2);
        merge!(user_3);
        merge!(user_4);
        merge!(any);
    }
    pub fn is_empty(&self) -> bool {
        macro_rules! check {
            ($($field:ident),*) => {
                $(self.$field.is_none())&&*
            };
        }

        check!(
            default, checked, focused, focus_key, edited, hovered, pressed, disabled, user_1,
            user_2, user_3, user_4, any
        )
    }
}

#[cfg(test)]
mod tests {
    use yaml_serde;

    use crate::serde_stylesheet::lv_types::LVAlign;
    use crate::serde_stylesheet::lv_types::LVColor;

    use super::*;

    #[test]
    fn test_style_serde() {
        let props_default = LVProperties {
            width: Some(100),
            bg_color: Some(LVColor::Rgb(212, 212, 212)),
            border_color: Some(LVColor::Rgb(191, 191, 191)),
            align: Some(LVAlign::Center),
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
