use std::marker::PhantomData;

use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct AllowContent;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct NoContent;

pub trait ContentPolicy {
    const ALLOW_CONTENT: bool;
}

impl ContentPolicy for AllowContent {
    const ALLOW_CONTENT: bool = true;
}

impl ContentPolicy for NoContent {
    const ALLOW_CONTENT: bool = false;
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Coord<C> {
    value: LVCoordValue,
    _marker: PhantomData<C>,
}

pub type LVSize = Coord<AllowContent>;
pub type LVCoord = Coord<NoContent>;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum LVCoordValue {
    Px(i32),
    Pct(i32),
    Content,
}

impl<C> Coord<C> {
    pub fn to_lvgl(&self) -> String {
        match self.value {
            LVCoordValue::Px(n) => n.to_string(),
            LVCoordValue::Pct(n) => format!("LV_PCT({})", n),
            LVCoordValue::Content => "LV_SIZE_CONTENT".to_string(),
        }
    }
}

impl<'de, C: ContentPolicy> Deserialize<'de> for Coord<C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.trim();

        if s == "content" || s == "LV_SIZE_CONTENT" {
            if !C::ALLOW_CONTENT {
                return Err(DeError::custom(
                    "LV_SIZE_CONTENT is not allowed for this property, use a pixel or percentage value",
                ));
            }
            return Ok(Coord {
                value: LVCoordValue::Content,
                _marker: PhantomData,
            });
        }

        if let Some(pct) = s.strip_suffix('%') {
            let n = pct
                .trim()
                .parse::<i32>()
                .map_err(|_| DeError::custom(format!("invalid percentage: {}", s)))?;
            return Ok(Coord {
                value: LVCoordValue::Pct(n),
                _marker: PhantomData,
            });
        }

        let n = s.parse::<i32>().map_err(|_| {
            DeError::custom(format!(
                "invalid coord: '{}', use '10', '10%' or 'content'",
                s
            ))
        })?;

        Ok(Coord {
            value: LVCoordValue::Px(n),
            _marker: PhantomData,
        })
    }
}

impl<C> Serialize for Coord<C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_lvgl())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lv_coord_px() {
        let v: Coord<AllowContent> = yaml_serde::from_str("42").unwrap();
        assert_eq!(v.value, LVCoordValue::Px(42));
        assert_eq!(v.to_lvgl(), "42");
    }

    #[test]
    fn test_lv_coord_pct() {
        let v: Coord<AllowContent> = yaml_serde::from_str("50%").unwrap();
        assert_eq!(v.value, LVCoordValue::Pct(50));
        assert_eq!(v.to_lvgl(), "LV_PCT(50)");
    }

    #[test]
    fn test_lv_coord_content() {
        let v: Coord<AllowContent> = yaml_serde::from_str("content").unwrap();
        assert_eq!(v.value, LVCoordValue::Content);
        assert_eq!(v.to_lvgl(), "LV_SIZE_CONTENT");

        let v: Coord<AllowContent> = yaml_serde::from_str("LV_SIZE_CONTENT").unwrap();
        assert_eq!(v.value, LVCoordValue::Content);
    }

    #[test]
    fn test_lv_coord_no_content() {
        let v: Result<Coord<NoContent>, _> = yaml_serde::from_str("content");
        assert!(v.is_err());

        let v: Result<Coord<NoContent>, _> = yaml_serde::from_str("LV_SIZE_CONTENT");
        assert!(v.is_err());
    }

    #[test]
    fn test_lv_dim_px_and_pct() {
        let v: Coord<NoContent> = yaml_serde::from_str("100").unwrap();
        assert_eq!(v.value, LVCoordValue::Px(100));

        let v: Coord<NoContent> = yaml_serde::from_str("25%").unwrap();
        assert_eq!(v.value, LVCoordValue::Pct(25));
    }

    #[test]
    fn test_lv_coord_invalid() {
        let v: Result<Coord<AllowContent>, _> = yaml_serde::from_str("invalid");
        assert!(v.is_err());

        let v: Result<Coord<AllowContent>, _> = yaml_serde::from_str("10px");
        assert!(v.is_err());
    }
}
