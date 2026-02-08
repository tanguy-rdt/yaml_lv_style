use serde::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[derive(Debug, PartialEq)]
pub enum LVState {
    LvStateDefault,
    LvStateChecked,
    LvStateFocused,
    LvStateFocusKey,
    LvStateEdited,
    LvStateHovered,
    LvStatePressed,
    LvStateDisabled,
    LvStateUser1,
    LvStateUser2,
    LvStateUser3,
    LvStateUser4,
    LvStateAny,
}

impl<'de> Deserialize<'de> for LVState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "default" | "LV_STATE_DEFAULT" => Ok(Self::LvStateDefault),
            "checked" | "LV_STATE_CHECKED" => Ok(Self::LvStateChecked),
            "focused" | "LV_STATE_FOCUSED" => Ok(Self::LvStateFocused),
            "focus_key" | "LV_STATE_FOCUS_KEY" => Ok(Self::LvStateFocusKey),
            "edited" | "LV_STATE_EDITED" => Ok(Self::LvStateEdited),
            "hovered" | "LV_STATE_HOVERED" => Ok(Self::LvStateHovered),
            "pressed" | "LV_STATE_PRESSED" => Ok(Self::LvStatePressed),
            "disabled" | "LV_STATE_DISABLED" => Ok(Self::LvStateDisabled),
            "user_1" | "LV_STATE_USER_1" => Ok(Self::LvStateUser1),
            "user_2" | "LV_STATE_USER_2" => Ok(Self::LvStateUser2),
            "user_3" | "LV_STATE_USER_3" => Ok(Self::LvStateUser3),
            "user_4" | "LV_STATE_USER_4" => Ok(Self::LvStateUser4),
            "any" | "LV_STATE_ANY" => Ok(Self::LvStateAny),
            other => Err(serde::de::Error::custom(format!("invalid state: {}", other))),
        }
    }
}

impl Serialize for LVState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LvStateDefault => "LV_STATE_DEFAULT",
            Self::LvStateChecked => "LV_STATE_CHECKED",
            Self::LvStateFocused => "LV_STATE_FOCUSED",
            Self::LvStateFocusKey => "LV_STATE_FOCUS_KEY",
            Self::LvStateEdited => "LV_STATE_EDITED",
            Self::LvStateHovered => "LV_STATE_HOVERED",
            Self::LvStatePressed => "LV_STATE_PRESSED",
            Self::LvStateDisabled => "LV_STATE_DISABLED",
            Self::LvStateUser1 => "LV_STATE_USER_1",
            Self::LvStateUser2 => "LV_STATE_USER_2",
            Self::LvStateUser3 => "LV_STATE_USER_3",
            Self::LvStateUser4 => "LV_STATE_USER_4",
            Self::LvStateAny => "LV_STATE_ANY",
        };
        serializer.serialize_str(s)
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
            assert_eq!(variant, parsed, "Comparison between serialisation and deserialisation failed for {:?}", variant);
        }
    }
}