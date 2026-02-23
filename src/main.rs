mod stylesheet;
mod generator;

use std::path::PathBuf;

use clap::Parser;

use crate::generator::generator::Generator;
use crate::stylesheet::stylesheet::StyleSheet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum Language {
    C,
    Cpp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ClangFormatStyle {
    LLVM,
    GNU,
    Google,
    Chromium,
    Microsoft,
    Mozilla,
    WebKit,
    File
}

impl ClangFormatStyle {
    pub fn to_clang_preset(&self) -> &str {
        match self {
            ClangFormatStyle::LLVM => "LLVM",
            ClangFormatStyle::Google => "Google",
            ClangFormatStyle::Chromium => "Chromium",
            ClangFormatStyle::Mozilla => "Mozilla",
            ClangFormatStyle::WebKit => "WebKit",
            ClangFormatStyle::Microsoft => "Microsoft",
            ClangFormatStyle::GNU => "GNU",
            ClangFormatStyle::File => "file",
        }
    }
}

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
        help = "Directory where the generated files will be saved (defaults to current directory).\n"
    )]
    output_dir: Option<PathBuf>,

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
        short,
        help = "Print the generated file paths to stdout\n",
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
    let mut generator = Generator::new(output_dir, opt.format);

    match opt.language {
        Language::C => generator.generate_c(&stylesheets),
        Language::Cpp => generator.generate_cpp(opt.namespace.as_deref(), &stylesheets),
    }.unwrap_or_else(|e| {
        log::error!("{}", e);
        std::process::exit(2);
    });

    if opt.print_gen_file_path {
        generator.print_generated_files_path();
    }
}