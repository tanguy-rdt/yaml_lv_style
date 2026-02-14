use serde::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

#[cfg_attr(test, derive(PartialEq, strum_macros::EnumIter))]
#[derive(Debug)]
pub enum LVFlexFlow {
    LVFlexFlowRow,
    LVFlexFlowColumn,
    LVFlexFlowRowWrap,
    LVFlexFlowRowReverse,
    LVFlexFlowRowWrapReverse,
    LVFlexFlowColumnWrap,
    LVFlexFlowColumnReverse,
    LVFlexFlowColumnWrapReverse,
}

impl<'de> Deserialize<'de> for LVFlexFlow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "row" | "LV_FLEX_FLOW_ROW" => Ok(Self::LVFlexFlowRow),
            "column" | "LV_FLEX_FLOW_COLUMN" => Ok(Self::LVFlexFlowColumn),
            "row_wrap" | "LV_FLEX_FLOW_ROW_WRAP" => Ok(Self::LVFlexFlowRowWrap),
            "row_reverse" | "LV_FLEX_FLOW_ROW_REVERSE" => Ok(Self::LVFlexFlowRowReverse),
            "row_wrap_reverse" | "LV_FLEX_FLOW_ROW_WRAP_REVERSE" => Ok(Self::LVFlexFlowRowWrapReverse),
            "column_wrap" | "LV_FLEX_FLOW_COLUMN_WRAP" => Ok(Self::LVFlexFlowColumnWrap),
            "column_reverse" | "LV_FLEX_FLOW_COLUMN_REVERSE" => Ok(Self::LVFlexFlowColumnReverse),
            "column_wrap_reverse" | "LV_FLEX_FLOW_COLUMN_WRAP_REVERSE" => Ok(Self::LVFlexFlowColumnWrapReverse),
            other => Err(serde::de::Error::custom(format!("invalid flex flow: {}", other))),
        }
    }
}

impl Serialize for LVFlexFlow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LVFlexFlowRow => "LV_FLEX_FLOW_ROW",
            Self::LVFlexFlowColumn => "LV_FLEX_FLOW_COLUMN",
            Self::LVFlexFlowRowWrap => "LV_FLEX_FLOW_ROW_WRAP",
            Self::LVFlexFlowRowReverse => "LV_FLEX_FLOW_ROW_REVERSE",
            Self::LVFlexFlowRowWrapReverse => "LV_FLEX_FLOW_ROW_WRAP_REVERSE",
            Self::LVFlexFlowColumnWrap => "LV_FLEX_FLOW_COLUMN_WRAP",
            Self::LVFlexFlowColumnReverse => "LV_FLEX_FLOW_COLUMN_REVERSE",
            Self::LVFlexFlowColumnWrapReverse => "LV_FLEX_FLOW_COLUMN_WRAP_REVERSE",
        };
        serializer.serialize_str(s)
    }
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
            assert_eq!(variant, parsed, "Comparison between serialisation and deserialisation failed for {:?}", variant);
        }
    }
}