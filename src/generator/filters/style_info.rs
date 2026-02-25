use std::collections::HashMap;

use tera::Result as TeraResult;
use tera::Value;

pub fn has_const_style(stylesheet: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let has_const = stylesheet
        .get("styles")
        .and_then(|v| v.as_array())
        .map(|styles| {
            styles
                .iter()
                .any(|style| style.get("const_style").and_then(|v| v.as_bool()) == Some(true))
        })
        .unwrap_or(false);

    Ok(Value::Bool(has_const))
}

pub fn has_dyn_style(stylesheet: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    let has_dyn = stylesheet
        .get("styles")
        .and_then(|v| v.as_array())
        .map(|styles| {
            styles
                .iter()
                .any(|style| style.get("const_style").and_then(|v| v.as_bool()) == Some(false))
        })
        .unwrap_or(false);

    Ok(Value::Bool(has_dyn))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tera::to_value;

    #[test]
    fn test_has_const_style() {
        let stylesheet = serde_json::json!({
            "styles": [
                { "name": "s1", "const_style": true },
                { "name": "s2", "const_style": false }
            ]
        });
        let val = to_value(stylesheet).unwrap();
        let result = has_const_style(&val, &HashMap::new()).unwrap();
        assert_eq!(result, Value::Bool(true));

        let stylesheet_dyn_only = serde_json::json!({
            "styles": [
                { "name": "s1", "const_style": false }
            ]
        });
        let val_dyn = to_value(stylesheet_dyn_only).unwrap();
        let result_dyn = has_const_style(&val_dyn, &HashMap::new()).unwrap();
        assert_eq!(result_dyn, Value::Bool(false));
    }

    #[test]
    fn test_has_dyn_style() {
        let stylesheet = serde_json::json!({
            "styles": [
                { "name": "s1", "const_style": true },
                { "name": "s2", "const_style": false }
            ]
        });
        let val = to_value(stylesheet).unwrap();
        let result = has_dyn_style(&val, &HashMap::new()).unwrap();
        assert_eq!(result, Value::Bool(true));

        let stylesheet_const_only = serde_json::json!({
            "styles": [
                { "name": "s1", "const_style": true }
            ]
        });
        let val_const = to_value(stylesheet_const_only).unwrap();
        let result_const = has_dyn_style(&val_const, &HashMap::new()).unwrap();
        assert_eq!(result_const, Value::Bool(false));
    }

    #[test]
    fn test_empty_stylesheet() {
        let empty_sheet = serde_json::json!({
            "styles": []
        });
        let val = to_value(empty_sheet).unwrap();

        assert_eq!(
            has_const_style(&val, &HashMap::new()).unwrap(),
            Value::Bool(false)
        );
        assert_eq!(
            has_dyn_style(&val, &HashMap::new()).unwrap(),
            Value::Bool(false)
        );
    }
}
