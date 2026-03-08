use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde::{Serialize, Serializer};

use crate::serde_stylesheet::types::{LVPart, LVState};

#[cfg_attr(test, derive(Debug))]
#[derive(PartialEq)]
pub struct LVSelector {
    pub state: LVState,
    pub part: LVPart,
}

impl LVSelector {
    pub fn to_lv(&self) -> String {
        format!(
            "{} | {}",
            String::from(&self.state),
            String::from(&self.part),
        )
    }

    pub fn to_snake_case(&self) -> String {
        format!(
            "{}_{}",
            self.state.to_snake_case(),
            self.part.to_snake_case(),
        )
    }
}

impl Default for LVSelector {
    fn default() -> Self {
        Self {
            state: LVState::Default,
            part: LVPart::Main,
        }
    }
}

impl FromStr for LVSelector {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.splitn(2, '.').collect::<Vec<_>>().as_slice() {
            [state] => Ok(Self {
                state: state.parse()?,
                part: LVPart::Main,
            }),
            [state, part] => Ok(Self {
                state: state.parse()?,
                part: part.parse()?,
            }),
            _ => Err(format!("Invalid selector: '{s}'")),
        }
    }
}

impl From<&LVSelector> for String {
    fn from(selector: &LVSelector) -> Self {
        format!(
            "{} | {}",
            String::from(&selector.state),
            String::from(&selector.part),
        )
    }
}

impl<'de> Deserialize<'de> for LVSelector {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        LVSelector::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for LVSelector {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("LVSelector", 2)?;
        s.serialize_field("lv", &self.to_lv())?;
        s.serialize_field("identifier", &self.to_snake_case())?;
        s.end()
    }
}
