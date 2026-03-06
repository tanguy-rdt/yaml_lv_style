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

impl From<String> for LVState {
    fn from(value: String) -> Self {
        match value.as_str() {
            "default" | "LV_STATE_DEFAULT" => LVState::Default,
            "checked" | "LV_STATE_CHECKED" => LVState::Checked,
            "focused" | "LV_STATE_FOCUSED" => LVState::Focused,
            "focus_key" | "LV_STATE_FOCUS_KEY" => LVState::FocusKey,
            "edited" | "LV_STATE_EDITED" => LVState::Edited,
            "hovered" | "LV_STATE_HOVERED" => LVState::Hovered,
            "pressed" | "LV_STATE_PRESSED" => LVState::Pressed,
            "disabled" | "LV_STATE_DISABLED" => LVState::Disabled,
            "user_1" | "LV_STATE_USER_1" => LVState::User1,
            "user_2" | "LV_STATE_USER_2" => LVState::User2,
            "user_3" | "LV_STATE_USER_3" => LVState::User3,
            "user_4" | "LV_STATE_USER_4" => LVState::User4,
            "any" | "LV_STATE_ANY" => LVState::Any,
            _ => LVState::Default,
        }
    }
}

impl From<&LVState> for String {
    fn from(state: &LVState) -> Self {
        match state {
            LVState::Default => "LV_STATE_DEFAULT".to_string(),
            LVState::Checked => "LV_STATE_CHECKED".to_string(),
            LVState::Focused => "LV_STATE_FOCUSED".to_string(),
            LVState::FocusKey => "LV_STATE_FOCUS_KEY".to_string(),
            LVState::Edited => "LV_STATE_EDITED".to_string(),
            LVState::Hovered => "LV_STATE_HOVERED".to_string(),
            LVState::Pressed => "LV_STATE_PRESSED".to_string(),
            LVState::Disabled => "LV_STATE_DISABLED".to_string(),
            LVState::User1 => "LV_STATE_USER_1".to_string(),
            LVState::User2 => "LV_STATE_USER_2".to_string(),
            LVState::User3 => "LV_STATE_USER_3".to_string(),
            LVState::User4 => "LV_STATE_USER_4".to_string(),
            LVState::Any => "LV_STATE_ANY".to_string(),
        }
    }
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
            assert_eq!(variant, parsed);
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
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_state_invalid() {
        let result: Result<LVState, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
