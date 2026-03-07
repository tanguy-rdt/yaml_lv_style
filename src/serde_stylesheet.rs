mod properties;
mod style;
mod stylesheet;
mod types;

use std::fs;
use std::path::Path;

pub use stylesheet::CanonicalStyleSheet;
use stylesheet::ParsedStyleSheet;
pub use types::LVState;

use crate::errors::Error;
use crate::errors::Result;

pub fn from_yaml(path: &Path) -> Result<CanonicalStyleSheet> {
    let yaml_str = fs::read_to_string(path).map_err(|e| Error::Io(e, path.to_path_buf()))?;
    let parsed_stylesheet = ParsedStyleSheet::deserialize_stylesheet(path, &yaml_str)?;
    Ok(CanonicalStyleSheet::from(parsed_stylesheet))
}
