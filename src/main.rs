mod config;
mod errors;
mod generator;
mod serde_stylesheet;

use config::Config;
use errors::Error;
use generator::Generator;
use generator::Language;

fn main() {
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("{:?}", miette::Report::new(e));
        std::process::exit(1);
    });

    let mut stylesheets = Vec::new();
    for yaml_stylesheet in config.input {
        match serde_stylesheet::from_yaml(&yaml_stylesheet) {
            Ok(stylesheet) => stylesheets.push(stylesheet),
            Err(e) => {
                eprintln!("{:?}", miette::Report::new(e));
                std::process::exit(2);
            }
        }
    }

    let mut generator = Generator::new(config.output_dir, config.format);

    match config.language {
        Language::C => generator.generate_c(&stylesheets),
        Language::Cpp => generator.generate_cpp(config.namespace.as_deref(), &stylesheets),
    }
    .unwrap_or_else(|e| {
        eprintln!("{:?}", miette::Report::new(e));
        std::process::exit(3);
    });

    if let Some(output_list) = config.output_list {
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
            std::process::exit(4);
        }
    }
}
