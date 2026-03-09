use serde::Deserialize;
use serde::{Serialize, Serializer};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Deserialize)]
pub struct LVSize {
    width: i32,
    height: i32,
}

impl Serialize for LVSize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}, {}", self.width, self.height))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lv_size_serde() {
        let yaml = r#"
            width: 50
            height: 100
            "#;
        let parsed: LVSize = yaml_serde::from_str(yaml).unwrap();
        let out = yaml_serde::to_string(&parsed).unwrap();
        let out = out.trim_end();
        assert_eq!(out, "50, 100");

        let yaml = r#"
            height: 100
            width: 200
            "#;
        let parsed: LVSize = yaml_serde::from_str(yaml).unwrap();
        let out = yaml_serde::to_string(&parsed).unwrap();
        let out = out.trim_end();
        assert_eq!(out, "200, 100");


        let yaml = r#"{ width: 100, height: 200 }"#;
        let parsed: LVSize = yaml_serde::from_str(yaml).unwrap();
        let out = yaml_serde::to_string(&parsed).unwrap();
        let out = out.trim_end();
        assert_eq!(out, "100, 200");
    }

    #[test]
    fn test_lv_size_deserialization_invalid_value() {
        let yaml = r#"
            width: 50px
            height: 100px
            "#;
        let result: Result<LVSize, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());

        let yaml = r#"[ 100, 200 ]"#;
        let result: Result<LVSize, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }
}