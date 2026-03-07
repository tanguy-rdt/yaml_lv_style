mod errors;
mod generator;
mod serde_stylesheet;

use std::path::PathBuf;

use clap::Parser;

use crate::errors::Error;
use crate::generator::ClangFormatStyle;
use crate::generator::Generator;
use crate::generator::Language;

#[derive(Parser)]
#[command(version = "1.0.0 (lvgl v9.4.0)")]
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
        default_value = ".",
        value_name = "DIR",
        help = "Directory where the generated files will be saved (defaults to current directory).\n"
    )]
    output_dir: PathBuf,

    #[arg(
        short,
        long,
        value_enum,
        hide_possible_values = true,
        help = "Language for the generated files \n\
                Possible values: [c, cpp] \n"
    )]
    language: Language,

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
    format: Option<ClangFormatStyle>,

    #[arg(
        short,
        long,
        value_name = "NAME",
        help = "Optional C++ namespace to wrap the generated code (e.g., 'ui::styles').\n"
    )]
    namespace: Option<String>,

    #[arg(
        long,
        value_name = "FILE",
        help = "Create al ist of all generated files to the specified file.\n"
    )]
    output_list: Option<PathBuf>,
}

fn main() {
    env_logger::init();

    let opt = Opt::parse();

    let mut stylesheets = Vec::new();
    for yaml_stylesheet in opt.input {
        match serde_stylesheet::from_yaml(&yaml_stylesheet) {
            Ok(stylesheet) => stylesheets.push(stylesheet),
            Err(e) => {
                eprintln!("{:?}", miette::Report::new(e));
                std::process::exit(1);
            }
        }
    }

    let mut generator = Generator::new(opt.output_dir, opt.format);

    match opt.language {
        Language::C => generator.generate_c(&stylesheets),
        Language::Cpp => generator.generate_cpp(opt.namespace.as_deref(), &stylesheets),
    }
    .unwrap_or_else(|e| {
        eprintln!("{:?}", miette::Report::new(e));
        std::process::exit(2);
    });

    if let Some(output_list) = opt.output_list {
        let content = generator
            .get_generated_headers_path()
            .iter()
            .chain(generator.get_generated_sources_path().iter())
            .map(|p| p.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\n");

        if let Err(e) = std::fs::write(&output_list, content).map_err(|e| Error::Io(e, output_list))
        {
            eprintln!("{:?}", miette::Report::new(e));
            std::process::exit(3);
        }
    }
}
