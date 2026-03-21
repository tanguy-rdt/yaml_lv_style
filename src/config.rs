mod cli;
mod file;

use std::collections::HashSet;
use std::path::PathBuf;

use crate::errors::Error;
use crate::errors::Result;
use crate::generator::ClangFormatStyle;
use crate::generator::Language;

use cli::Cli;
use file::ConfigFile;

const DEFAULT_OUTPUT_DIR: &str = ".";
const DEFAULT_LANGUAGE: Language = Language::C;

pub struct Config {
    pub input: Vec<PathBuf>,
    pub output_dir: PathBuf,
    pub language: Language,
    pub namespace: Option<String>,
    pub format: Option<ClangFormatStyle>,
    pub output_list: Option<PathBuf>,
}

impl Config {
    pub fn load() -> Result<Self> {
        use clap::Parser;
        let mut cli = Cli::parse();

        let config_file = cli
            .config
            .take()
            .map(|p| ConfigFile::load(&p))
            .transpose()?
            .unwrap_or_default();

        Ok(Config {
            input: get_inputs(cli.input, config_file.input)?,
            output_dir: get_output_dir(cli.output_dir, config_file.output_dir),
            language: get_language(cli.language, config_file.language),
            namespace: get_namespace(cli.namespace, config_file.namespace),
            format: get_format(cli.format, config_file.format),
            output_list: cli.output_list,
        })
    }
}

fn get_inputs(cli_input: Vec<PathBuf>, file_input: Option<Vec<PathBuf>>) -> Result<Vec<PathBuf>> {
    let inputs: Vec<PathBuf> = cli_input
        .into_iter()
        .chain(file_input.into_iter().flatten())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    (!inputs.is_empty())
        .then_some(inputs)
        .ok_or_else(|| Error::Config("You must specify at least one YAML stylesheet".to_string()))
}

fn get_output_dir(cli_output_dir: Option<PathBuf>, file_output_dir: Option<PathBuf>) -> PathBuf {
    cli_output_dir.unwrap_or_else(|| file_output_dir.unwrap_or_else(|| DEFAULT_OUTPUT_DIR.into()))
}

fn get_language(cli_language: Option<Language>, file_language: Option<Language>) -> Language {
    cli_language.unwrap_or_else(|| file_language.unwrap_or(DEFAULT_LANGUAGE))
}

fn get_namespace(cli_namespace: Option<String>, file_namespace: Option<String>) -> Option<String> {
    cli_namespace.or(file_namespace)
}

fn get_format(
    cli_format: Option<ClangFormatStyle>,
    file_format: Option<ClangFormatStyle>,
) -> Option<ClangFormatStyle> {
    cli_format.or(file_format)
}
