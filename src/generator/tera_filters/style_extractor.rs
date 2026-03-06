use std::collections::HashMap;

use tera::Result as TeraResult;
use tera::Value;

use crate::serde_stylesheet::LVState;

pub fn get_states_sorted(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let array = match value {
        Value::Array(arr) => arr,
        _ => {
            return Err(tera::Error::msg(
                "Unable to extract the properties of the state",
            ));
        }
    };

    let mut sorted: Vec<&String> = array
        .iter()
        .filter_map(|item| match item {
            Value::Array(pair) if pair.len() == 2 => match (&pair[0], &pair[1]) {
                (Value::String(k), _) => Some(k),
                _ => None,
            },
            _ => None,
        })
        .collect();

    sorted.sort();

    Ok(tera::to_value(sorted)?)
}

#[allow(dead_code)]
pub fn get_props_by_states(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let mut map = match value {
        Value::Object(map) => map.clone(),
        _ => {
            log::warn!("Unable to extract the properties by states");
            return Ok(Value::Null);
        }
    };

    map.remove("name");
    map.remove("const_style");
    map.remove("declarations");
    map.retain(|_, v| !v.is_null());

    Ok(tera::to_value(map)?)
}

pub fn get_props_by_states_sorted(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let mut map = match value {
        Value::Object(map) => map.clone(),
        _ => {
            log::warn!("Unable to extract the properties by states");
            return Ok(Value::Null);
        }
    };

    map.remove("name");
    map.remove("const_style");
    map.remove("declarations");

    let mut sorted_props: Vec<(&String, &Value)> =
        map.iter().filter(|(_, v)| !v.is_null()).collect();

    sorted_props.sort_by(|a, b| a.0.cmp(b.0));

    Ok(tera::to_value(sorted_props)?)
}

#[allow(dead_code)]
pub fn get_props(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let mut map = match value {
        Value::Object(map) => map.clone(),
        _ => {
            log::warn!("Unable to extract the properties of the state");
            return Ok(Value::Null);
        }
    };

    map.retain(|_, v| !v.is_null());

    Ok(tera::to_value(map)?)
}

pub fn get_props_sorted(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let array = match value {
        Value::Array(arr) => arr,
        _ => {
            return Err(tera::Error::msg(
                "Unable to extract the properties of the state",
            ));
        }
    };

    let mut sorted: Vec<(&String, &Value)> = array
        .iter()
        .filter_map(|item| match item {
            Value::Array(pair) if pair.len() == 2 => match (&pair[0], &pair[1]) {
                (Value::String(k), v) => Some((k, v)),
                _ => None,
            },
            _ => None,
        })
        .collect();

    sorted.sort_by(|a, b| a.0.cmp(b.0));

    Ok(tera::to_value(sorted)?)
}

pub fn get_lv_state(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let str = match value.as_str() {
        Some(str) => str.to_string(),
        None => {
            log::warn!("Unable to extract the state");
            return Ok(Value::Null);
        }
    };

    match yaml_serde::from_str::<LVState>(str.trim()) {
        Ok(lv_state) => Ok(tera::to_value(lv_state)?),
        Err(_) => {
            log::warn!("Unable to convert the state to a lvgl state");
            Ok(Value::Null)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tera::to_value;

    #[test]
    fn test_get_states_of_style() {
        let style = json!({
            "name": "btn",
            "const_style": true,
            "default": {"align": "align"},
            "checked": null,
            "focus_key": {"align": "align"},
            "edited": {"align": "align"},
            "hovered": {"align": "align"},
            "pressed": null
        });

        let result = get_states_sorted(&style, &HashMap::new()).unwrap();
        let mut states: Vec<String> = serde_json::from_value(result).unwrap();
        states.sort();

        assert_eq!(states.len(), 4);
        assert!(!states.contains(&"name".to_string()));
        assert!(!states.contains(&"const_style".to_string()));
        assert!(states.contains(&"default".to_string()));
        assert!(!states.contains(&"checked".to_string()));
        assert!(states.contains(&"focus_key".to_string()));
        assert!(states.contains(&"edited".to_string()));
        assert!(states.contains(&"hovered".to_string()));
        assert!(!states.contains(&"pressed".to_string()));
    }

    #[test]
    fn test_get_props_by_states_sorted() {
        let style = json!({
            "name": "test",
            "const_style": false,
            "any": null,
            "default": {"align": "align"},
            "checked": {"align": "align"},
            "focus_key": {"align": "align"},
            "edited": {"align": "align"},
            "hovered": {"align": "align"},
        });

        let result = get_props_by_states_sorted(&style, &HashMap::new()).unwrap();
        let sorted_list: Vec<(String, Value)> = serde_json::from_value(result).unwrap();

        assert_eq!(sorted_list.len(), 5);
        assert_eq!(sorted_list[0].0, "checked");
        assert_eq!(sorted_list[1].0, "default");
        assert_eq!(sorted_list[2].0, "edited");
        assert_eq!(sorted_list[3].0, "focus_key");
        assert_eq!(sorted_list[4].0, "hovered");
    }

    #[test]
    fn test_get_props_sorted() {
        let props = json!({
            "width": "width",
            "align": "align",
            "bg_color": "bg_color"
        });

        let result = get_props_sorted(&props, &HashMap::new()).unwrap();
        let sorted_props: Vec<(String, Value)> = serde_json::from_value(result).unwrap();

        assert_eq!(sorted_props[0].0, "align");
        assert_eq!(sorted_props[1].0, "bg_color");
        assert_eq!(sorted_props[2].0, "width");
    }

    #[test]
    fn test_get_lv_state() {
        let state_val = to_value("checked").unwrap();
        let result = get_lv_state(&state_val, &HashMap::new()).unwrap();
        assert_eq!(result.as_str().unwrap(), "LV_STATE_CHECKED");

        let invalid_val = to_value("invalid_state").unwrap();
        let result_err = get_lv_state(&invalid_val, &HashMap::new()).unwrap();
        assert!(result_err.is_null());
    }
}
