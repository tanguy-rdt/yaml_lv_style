use std::collections::HashMap;

use tera::Result as TeraResult;
use tera::Value;

use crate::serde_stylesheet::LVState;

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
    use tera::to_value;

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
