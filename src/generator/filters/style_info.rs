use std::collections::HashMap;

use tera::Value;
use tera::Result as TeraResult;

pub fn has_const_style(stylesheet: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let has_const = stylesheet
        .get("styles")
        .and_then(|v| v.as_array())
        .map(|styles| {
            styles.iter().any(|style| {
                style
                    .get("const_style")
                    .and_then(|v| v.as_bool())
                    == Some(true)
            })
        })
        .unwrap_or(false);

    Ok(Value::Bool(has_const))
}

pub fn has_dyn_style(stylesheet: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let has_dyn = stylesheet
        .get("styles")
        .and_then(|v| v.as_array())
        .map(|styles| {
            styles.iter().any(|style| {
                style
                    .get("const_style")
                    .and_then(|v| v.as_bool())
                    == Some(false)
            })
        })
        .unwrap_or(false);

    Ok(Value::Bool(has_dyn))
}