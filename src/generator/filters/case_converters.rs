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

#[cfg(test)]
mod tests {
    use super::*;
    use tera::to_value;

    #[test]
    fn test_pascal_case_filter() {
        let tests = vec![
            ("style_name", "StyleName"),
            ("bg_color", "BgColor"),
            ("width", "Width"),
            ("", ""),
            ("_leading", "Leading"),
            ("trailing_", "Trailing"),
        ];

        for (input, expected) in tests {
            let result = pascal_case_filter(&to_value(input).unwrap(), &HashMap::new()).unwrap();
            assert_eq!(result, to_value(expected).unwrap());
        }
    }

    #[test]
    fn test_screaming_snake_case_filter() {
        let tests = vec![
            ("style_name", "STYLE_NAME"),
            ("bg_color", "BG_COLOR"),
            ("width", "WIDTH"),
            ("", ""),
        ];

        for (input, expected) in tests {
            let result = screaming_snake_case_filter(&to_value(input).unwrap(), &HashMap::new()).unwrap();
            assert_eq!(result, to_value(expected).unwrap());
        }
    }

    #[test]
    fn test_snake_case_filter() {
        let tests = vec![
            ("StyleName", "stylename"),
            ("BG_COLOR", "bg_color"),
            ("Width", "width"),
            ("", ""),
        ];

        for (input, expected) in tests {
            let result = snake_case_filter(&to_value(input).unwrap(), &HashMap::new()).unwrap();
            assert_eq!(result, to_value(expected).unwrap());
        }
    }
}