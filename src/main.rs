mod stylesheet;

use clap::Parser;

use crate::stylesheet::stylesheet::StyleSheet;

#[derive(Parser)]
struct Opt {
    #[clap(required = true, long)]
    yaml_stylesheets: Vec<String>,
    #[clap(long)]
    namespace: Option<String>,
    #[clap(long)]
    output_path: Option<String>,
}



fn main() {
    env_logger::init();

    let opt = Opt::parse();

    let mut stylesheets = Vec::new();
    for yaml_stylesheet in opt.yaml_stylesheets {
        match StyleSheet::from_yaml(&yaml_stylesheet) {
            Ok(stylesheet) =>  stylesheets.push(stylesheet),
            Err(e) => {
                log::error!("{}", e);
                std::process::exit(1);
            }
        }
    }
}