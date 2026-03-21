use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::Error;
use crate::errors::Result;
use crate::generator::{ClangFormatStyle, Language};

#[derive(Deserialize, Default)]
pub struct ConfigFile {
    pub input: Option<Vec<PathBuf>>,
    pub output_dir: Option<PathBuf>,
    pub language: Option<Language>,
    pub namespace: Option<String>,
    pub format: Option<ClangFormatStyle>,
}

impl ConfigFile {
    pub fn load(path: &Path) -> Result<ConfigFile> {
        let config_str = fs::read_to_string(path).map_err(|e| Error::Io(e, path.to_path_buf()))?;
        let mut config: ConfigFile = yaml_serde::from_str(&config_str)
            .map_err(|e| Error::yaml_serde(e, path.to_path_buf(), config_str.to_string()))?;

        let config_dir = path.parent().unwrap_or(Path::new("."));
        config.input = config.input.map(|paths| {
            paths
                .into_iter()
                .map(|p| {
                    if p.is_relative() {
                        config_dir.join(&p)
                    } else {
                        p
                    }
                })
                .collect()
        });

        Ok(config)
    }
}
