use serde::{Deserialize, Serialize};

#[cfg_attr(test, derive(Debug, PartialEq, strum_macros::EnumIter))]
#[derive(Deserialize, Serialize)]
pub enum LVFlexFlow {
    #[serde(rename = "LV_FLEX_FLOW_ROW", alias = "row")]
    Row,
    #[serde(rename = "LV_FLEX_FLOW_COLUMN", alias = "column")]
    Column,
    #[serde(rename = "LV_FLEX_FLOW_ROW_WRAP", alias = "row_wrap")]
    RowWrap,
    #[serde(rename = "LV_FLEX_FLOW_ROW_REVERSE", alias = "row_reverse")]
    RowReverse,
    #[serde(rename = "LV_FLEX_FLOW_ROW_WRAP_REVERSE", alias = "row_wrap_reverse")]
    RowWrapReverse,
    #[serde(rename = "LV_FLEX_FLOW_COLUMN_WRAP", alias = "column_wrap")]
    ColumnWrap,
    #[serde(rename = "LV_FLEX_FLOW_COLUMN_REVERSE", alias = "column_reverse")]
    ColumnReverse,
    #[serde(
        rename = "LV_FLEX_FLOW_COLUMN_WRAP_REVERSE",
        alias = "column_wrap_reverse"
    )]
    ColumnWrapReverse,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_flex_flow_serde() {
        for variant in LVFlexFlow::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVFlexFlow = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed);
        }
    }

    #[test]
    fn test_lv_flex_flow_aliases() {
        let aliases = vec![
            ("row", LVFlexFlow::Row),
            ("column", LVFlexFlow::Column),
            ("row_wrap", LVFlexFlow::RowWrap),
            ("row_reverse", LVFlexFlow::RowReverse),
            ("row_wrap_reverse", LVFlexFlow::RowWrapReverse),
            ("column_wrap", LVFlexFlow::ColumnWrap),
            ("column_reverse", LVFlexFlow::ColumnReverse),
            ("column_wrap_reverse", LVFlexFlow::ColumnWrapReverse),
        ];

        assert_eq!(aliases.len(), LVFlexFlow::iter().count());

        for (alias, expected_variant) in aliases {
            let parsed: LVFlexFlow = yaml_serde::from_str(alias).unwrap();
            assert_eq!(parsed, expected_variant);
        }
    }

    #[test]
    fn test_lv_flex_flow_invalid() {
        let result: Result<LVFlexFlow, _> = yaml_serde::from_str("not_a_value");
        assert!(result.is_err());
    }
}
