mod stylesheet;
mod generator;

use std::path::PathBuf;

use clap::Parser;

use crate::generator::generator::generate;
use crate::stylesheet::stylesheet::StyleSheet;

#[derive(Parser)]
#[command(version)]
struct Opt {
    #[arg(
        short,
        long,
        required = true,
        num_args = 1..,
        value_name = "FILE",
        help = "Path to the input YAML stylesheet files to be processed.\n"
    )]
    input: Vec<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "DIR",
        help = "Directory where the generated C++ files will be saved (defaults to current directory).\n"
    )]
    output_dir: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "Optional C++ namespace to wrap the generated code (e.g., 'ui::styles').\n"
    )]
    namespace: Option<String>,

    #[arg(
        short = 'f',
        long,
        num_args = 0..=1,
        default_missing_value = "file",
        value_name = "STYLE",
        help = "Format the generated code with clang-format (if available). \n\n\
                Accepted styles: LLVM, Google, Chromium, Mozilla, WebKit, Microsoft, GNU.\n\
                If STYLE is omitted, it defaults to 'file' and looks for a .clang-format \n\
                file in the parent directories of the generated files.\n"
    )]
    format: Option<String>,

    #[arg(
        short = 'p',
        help = "Print the generated file paths to stdout",
    )]
    print_gen_file_path: bool,
}

fn main() {
    env_logger::init();

    let opt = Opt::parse();

    let mut stylesheets = Vec::new();
    for yaml_stylesheet in opt.input {
        match StyleSheet::from_yaml(&yaml_stylesheet) {
            Ok(stylesheet) =>  stylesheets.push(stylesheet),
            Err(e) => {
                log::error!("{}", e);
                std::process::exit(1);
            }
        }
    }

    let output_dir = opt.output_dir.unwrap_or_else(|| ".".into());
    if let Err(e) = generate(&stylesheets, &output_dir, &opt.namespace, &opt.format, opt.print_gen_file_path) {
        log::error!("{}", e);
        std::process::exit(2);
    }
}