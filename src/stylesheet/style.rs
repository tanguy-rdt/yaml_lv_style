use serde::{Deserialize, Serialize};

use super::lv_properties::LVProperties;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Style {
    pub name: String,
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