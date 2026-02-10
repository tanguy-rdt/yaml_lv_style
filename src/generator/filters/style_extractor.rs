use std::collections::HashMap;

use tera::Value;
use tera::Result as TeraResult;

use crate::stylesheet::lv_types::lv_state::LVState;

pub fn get_states_of_style(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let mut map = match value {
        Value::Object(map) => map.clone(),
        _ => {
            log::warn!("Unable to extract style states");
            return Ok(Value::Null)
        },
    };

    map.remove("name");
    map.remove("const_style");
    map.retain(|_, v| !v.is_null());

    let states: Vec<String> = map.keys().cloned().collect();

    Ok(tera::to_value(states)?)
}

pub fn get_props_by_state(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let mut map = match value {
        Value::Object(map) => map.clone(),
        _ => {
            log::warn!("Unable to extract the properties by states");
            return Ok(Value::Null)
        },
    };

    map.remove("name");
    map.remove("const_style");
    map.retain(|_, v| !v.is_null());

    Ok(tera::to_value(map)?)
}

pub fn get_props(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let mut map = match value {
        Value::Object(map) => map.clone(),
        _ => {
            log::warn!("Unable to extract the properties of the state");
            return Ok(Value::Null)
        },
    };

    map.retain(|_, v| !v.is_null());

    Ok(tera::to_value(map)?)
}

pub fn get_lv_state(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let str =  match value.as_str() {
        Some(str)=> str.to_string(),
        None => {
            log::warn!("Unable to extract the state");
            return Ok(Value::Null)
        },
    };

    match yaml_serde::from_str::<LVState>(str.trim()) {
        Ok(lv_state) => Ok(tera::to_value(lv_state)?),
        Err(_) => {
            log::warn!("Unable to convert the state to a lvgl state");
            Ok(Value::Null)
        },
    }
}