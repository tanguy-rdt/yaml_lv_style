use serde::{Deserialize, Serialize};

use super::lv_properties::LVProperties;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Default, Deserialize, Serialize)]
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
    pub fn make_properties_const(&mut self) {
        macro_rules! make_const {
            ($field:ident) => {
                if let Some(val) = &mut self.$field {
                    val.make_properties_const();
                }
            };
        }

        make_const!(default);
        make_const!(checked);
        make_const!(focused);
        make_const!(focus_key);
        make_const!(edited);
        make_const!(hovered);
        make_const!(pressed);
        make_const!(disabled);
        make_const!(user_1);
        make_const!(user_2);
        make_const!(user_3);
        make_const!(user_4);
        make_const!(any);
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
            align: Some(LVAlign::Center),
            ..Default::default()
        };

        let props_hovered = LVProperties {
            width: Some(10),
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
