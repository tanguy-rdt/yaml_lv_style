use serde::{Serialize, Serializer};
use serde::{Deserialize, Deserializer};

#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[derive(Debug, PartialEq)]
pub enum LVGradDir {
    LVGradDirNone,
    LVGradDirVer,
    LVGradDirHor,
    LVGradDirLinear,
    LVGradDirRadial,
    LVGradDirConical,
}

impl<'de> Deserialize<'de> for LVGradDir {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "none" | "LV_GRAD_DIR_NONE" => Ok(Self::LVGradDirNone),
            "vertical" | "LV_GRAD_DIR_VER" => Ok(Self::LVGradDirVer),
            "horizontal" | "LV_GRAD_DIR_HOR" => Ok(Self::LVGradDirHor),
            "linear" | "LV_GRAD_DIR_LINEAR" => Ok(Self::LVGradDirLinear),
            "radial" | "LV_GRAD_DIR_RADIAL" => Ok(Self::LVGradDirRadial),
            "conical" | "LV_GRAD_DIR_CONICAL" => Ok(Self::LVGradDirConical),
            other => Err(serde::de::Error::custom(format!("invalid grad dir: {}", other))),
        }
    }
}

impl Serialize for LVGradDir {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match self {
            Self::LVGradDirNone => "LV_GRAD_DIR_NONE",
            Self::LVGradDirVer => "LV_GRAD_DIR_VER",
            Self::LVGradDirHor => "LV_GRAD_DIR_HOR",
            Self::LVGradDirLinear => "LV_GRAD_DIR_LINEAR",
            Self::LVGradDirRadial => "LV_GRAD_DIR_RADIAL",
            Self::LVGradDirConical => "LV_GRAD_DIR_CONICAL",
        };
        serializer.serialize_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_lv_grad_dir_serde() {
        for variant in LVGradDir::iter() {
            let serialized = yaml_serde::to_string(&variant).unwrap();
            let parsed: LVGradDir = yaml_serde::from_str(serialized.trim()).unwrap();
            assert_eq!(variant, parsed, "Comparison between serialisation and deserialisation failed for {:?}", variant);
        }
    }
}