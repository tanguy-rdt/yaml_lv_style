use tera::Value;
use tera::Result as TeraResult;

use std::collections::HashMap;

pub fn pascal_case_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().unwrap_or("");

    let result = s
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>();

    Ok(Value::String(result))
}

pub fn screaming_snake_case_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().unwrap_or("");
    Ok(Value::String(s.to_uppercase()))
}

pub fn snake_case_filter(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = value.as_str().unwrap_or("");
    Ok(Value::String(s.to_lowercase()))
}
