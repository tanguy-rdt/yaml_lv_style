use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[serde(untagged)]
pub enum CodeStyle {
    SnakeCase,
    PascalCase,
}
