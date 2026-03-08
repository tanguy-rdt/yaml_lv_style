use std::path::PathBuf;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("IO Error: {0} at {1:?}")]
    #[diagnostic(
        code(yaml_lv_style::io_error),
        help("Check if the file exists and is readable.")
    )]
    Io(std::io::Error, PathBuf),

    #[error("IO Error: {0} at {1:?}")]
    #[diagnostic(
        code(yaml_lv_style::io_error),
        help("Check if the file exists and is readable.")
    )]
    IoKind(std::io::ErrorKind, PathBuf),

    // Clippy generates an error for an unused assignment in macros.
    // https://github.com/rust-lang/rust/issues/147648
    // https://github.com/zkat/miette/pull/459
    #[error("YAML Parsing Error")] // TODO remove _ and add path
    #[diagnostic(
        code(yaml_lv_style::yaml_syntax_error),
        help("See documentation") // TODO add doc link
    )]
    YamlSerde {
        #[source]
        _src_error: Box<Error>,
        #[source_code]
        _src: String,
        #[label("here")]
        _span: Option<SourceSpan>,
        _path: PathBuf,
    },

    #[error("Style {0} is empty in {1:?}")]
    #[diagnostic(
        code(yaml_lv_style::empty_style),
        help("A style must contain at least one selector and at least one property")
    )]
    EmptyStyle(String, PathBuf),

    #[error("Style {0} is defined twice in {1:?}")]
    #[diagnostic(code(yaml_lv_style::duplicated_style), help("A style must be unique"))]
    DuplicatedStyle(String, PathBuf),

    #[error("Selector {0} as no properties")]
    #[diagnostic(
        code(yaml_lv_style::empty_selector),
        help("A selector must contain at least one property")
    )]
    EmptySelector(String),

    #[error("Selector {0} is defined twice")]
    #[diagnostic(
        code(yaml_lv_style::duplicated_selector),
        help("A selector must be unique")
    )]
    DuplicatedSelector(String),

    #[error("Generation failed")]
    #[diagnostic(code(yaml_lv_style::generation_error))]
    Generation(#[source] Box<Error>),

    #[error("Rendering failed: {0}")]
    #[diagnostic(code(yaml_lv_style::render_error))]
    Render(String),

    #[error("Formatting error (clang-format): {0}")]
    #[diagnostic(
        code(yaml_lv_style::format_error),
        help("Check that clang-format is installed and accessible.")
    )]
    Format(String),

    #[error("{0}")]
    #[diagnostic(code(yaml_lv_style::other_error))]
    Other(String),
}

impl Error {
    pub fn yaml_serde(e: yaml_serde::Error, path: PathBuf, src: String) -> Self {
        let msg = e.to_string();

        let message = if let Some(idx) = msg.find(", expected one of") {
            msg[..idx].to_string()
        } else {
            msg
        };

        let span = e.location().map(|loc| SourceSpan::from((loc.index(), 0)));

        Self::YamlSerde {
            _src_error: Box::new(Error::Other(message)),
            _src: src,
            _span: span,
            _path: path,
        }
    }
}
