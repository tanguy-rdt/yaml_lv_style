use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVState {
    #[serde(rename = "LV_STATE_DEFAULT", alias = "default")]
    Default,
    #[serde(rename = "LV_STATE_CHECKED", alias = "checked")]
    Checked,
    #[serde(rename = "LV_STATE_FOCUSED", alias = "focused")]
    Focused,
    #[serde(rename = "LV_STATE_FOCUS_KEY", alias = "focus_key")]
    FocusKey,
    #[serde(rename = "LV_STATE_EDITED", alias = "edited")]
    Edited,
    #[serde(rename = "LV_STATE_HOVERED", alias = "hovered")]
    Hovered,
    #[serde(rename = "LV_STATE_PRESSED", alias = "pressed")]
    Pressed,
    #[serde(rename = "LV_STATE_DISABLED", alias = "disabled")]
    Disabled,
    #[serde(rename = "LV_STATE_USER_1", alias = "user_1")]
    User1,
    #[serde(rename = "LV_STATE_USER_2", alias = "user_2")]
    User2,
    #[serde(rename = "LV_STATE_USER_3", alias = "user_3")]
    User3,
    #[serde(rename = "LV_STATE_USER_4", alias = "user_4")]
    User4,
    #[serde(rename = "LV_STATE_ANY", alias = "any")]
    Any,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_state_serde() {
        for variant in LVState::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVState = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(
                variant, parsed,
                "Comparison between serialisation and deserialisation failed for {:?}",
                variant
            );
        }
    }

    #[test]
    fn test_lv_state_aliases() {
        let aliases = vec![
            ("default", LVState::Default),
            ("checked", LVState::Checked),
            ("focused", LVState::Focused),
            ("focus_key", LVState::FocusKey),
            ("edited", LVState::Edited),
            ("hovered", LVState::Hovered),
            ("pressed", LVState::Pressed),
            ("disabled", LVState::Disabled),
            ("user_1", LVState::User1),
            ("user_2", LVState::User2),
            ("user_3", LVState::User3),
            ("user_4", LVState::User4),
            ("any", LVState::Any),
        ];

        assert_eq!(aliases.len(), LVState::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVState = yaml_serde::from_str(alias).unwrap();
            assert_eq!(
                parsed, expected_variant,
                "The alias ‘{}’ was not correctly deserialized to {:?}",
                alias, expected_variant
            );
        }
    }
}
