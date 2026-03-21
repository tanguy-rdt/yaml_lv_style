use std::path::PathBuf;

use clap::Parser;

use crate::generator::{ClangFormatStyle, Language};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    #[arg(
        short,
        long,
        num_args = 1..,
        value_name = "FILE",
        required_unless_present = "config",
        help = "Path to the input YAML stylesheet files to be processed.\n"
    )]
    pub input: Vec<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "FILE",
        required_unless_present = "input",
        help = "Path to the YAML config file.\n"
    )]
    pub config: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "DIR",
        help = "Directory where the generated files will be saved (defaults to current directory).\n"
    )]
    pub output_dir: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_enum,
        hide_possible_values = true,
        help = "Language for the generated files \n\
                Possible values: [c, cpp] \n"
    )]
    pub language: Option<Language>,

    #[arg(
        short,
        long,
        value_name = "NAMESPACE",
        help = "Optional C++ namespace to wrap the generated code (e.g., 'ui::styles').\n"
    )]
    pub namespace: Option<String>,

    #[arg(
        short,
        long,
        num_args = 0..=1,
        value_enum,
        hide_possible_values = true,
        default_missing_value = "file",
        help = "Format the generated code with clang-format (if available on your system). \n\
                Default value: file \n\
                Possible values: [llvm, gnu, google, chromium, microsoft, mozilla, webkit, file] \n"
    )]
    pub format: Option<ClangFormatStyle>,

    #[arg(
        long,
        value_name = "FILE",
        help = "Create a list of all generated files to the specified file.\n"
    )]
    pub output_list: Option<PathBuf>,
}
