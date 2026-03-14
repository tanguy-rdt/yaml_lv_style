use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, strum_macros::EnumIter))]
#[derive(PartialEq, Deserialize, Serialize)]
pub enum LVState {
    #[serde(rename = "LV_STATE_DEFAULT", alias = "default")]
    Default,
    #[serde(rename = "LV_STATE_ALT", alias = "alt")]
    Alt,
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
    #[serde(rename = "LV_STATE_SCROLLED", alias = "scrolled")]
    Scrolled,
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

impl LVState {
    pub fn to_snake_case(&self) -> &str {
        match self {
            LVState::Default => "default",
            LVState::Alt => "alt",
            LVState::Checked => "checked",
            LVState::Focused => "focused",
            LVState::FocusKey => "focus_key",
            LVState::Edited => "edited",
            LVState::Hovered => "hovered",
            LVState::Pressed => "pressed",
            LVState::Scrolled => "scrolled",
            LVState::Disabled => "disabled",
            LVState::User1 => "user_1",
            LVState::User2 => "user_2",
            LVState::User3 => "user_3",
            LVState::User4 => "user_4",
            LVState::Any => "any",
        }
    }
}

impl FromStr for LVState {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "default" | "LV_STATE_DEFAULT" => Ok(LVState::Default),
            "alt" | "LV_STATE_ALT" => Ok(LVState::Alt),
            "checked" | "LV_STATE_CHECKED" => Ok(LVState::Checked),
            "focused" | "LV_STATE_FOCUSED" => Ok(LVState::Focused),
            "focus_key" | "LV_STATE_FOCUS_KEY" => Ok(LVState::FocusKey),
            "edited" | "LV_STATE_EDITED" => Ok(LVState::Edited),
            "hovered" | "LV_STATE_HOVERED" => Ok(LVState::Hovered),
            "pressed" | "LV_STATE_PRESSED" => Ok(LVState::Pressed),
            "scrolled" | "LV_STATE_SCROLLED" => Ok(LVState::Scrolled),
            "disabled" | "LV_STATE_DISABLED" => Ok(LVState::Disabled),
            "user_1" | "LV_STATE_USER_1" => Ok(LVState::User1),
            "user_2" | "LV_STATE_USER_2" => Ok(LVState::User2),
            "user_3" | "LV_STATE_USER_3" => Ok(LVState::User3),
            "user_4" | "LV_STATE_USER_4" => Ok(LVState::User4),
            "any" | "LV_STATE_ANY" => Ok(LVState::Any),
            _ => Err(format!("Unknown LVState: '{value}'")),
        }
    }
}

impl From<&LVState> for String {
    fn from(state: &LVState) -> Self {
        match state {
            LVState::Default => "LV_STATE_DEFAULT".to_string(),
            LVState::Alt => "LV_STATE_ALT".to_string(),
            LVState::Checked => "LV_STATE_CHECKED".to_string(),
            LVState::Focused => "LV_STATE_FOCUSED".to_string(),
            LVState::FocusKey => "LV_STATE_FOCUS_KEY".to_string(),
            LVState::Edited => "LV_STATE_EDITED".to_string(),
            LVState::Hovered => "LV_STATE_HOVERED".to_string(),
            LVState::Pressed => "LV_STATE_PRESSED".to_string(),
            LVState::Scrolled => "LV_STATE_SCROLLED".to_string(),
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
            ("alt", LVState::Alt),
            ("checked", LVState::Checked),
            ("focused", LVState::Focused),
            ("focus_key", LVState::FocusKey),
            ("edited", LVState::Edited),
            ("hovered", LVState::Hovered),
            ("pressed", LVState::Pressed),
            ("scrolled", LVState::Scrolled),
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
