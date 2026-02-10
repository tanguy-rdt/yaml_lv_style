use std::fs;
use std::io::Write;

use tera::Tera;

use super::filters::case_converters;
use super::filters::style_extractor;

use crate::stylesheet::stylesheet::StyleSheet;

pub fn generate(namespace: Option<String>, stylesheets: &Vec<StyleSheet>) -> Result<(), String> {
    let mut tera = match Tera::new("src/generator/templates/*.*.j2") {
        Ok(t) => t,
        Err(e) => return Err(format!("An error occurred while loading the Tera templates: {}", e)),
    };

    tera.register_filter("pascal_case", case_converters::pascal_case_filter);
    tera.register_filter("screaming_snake_case", case_converters::screaming_snake_case_filter);
    tera.register_filter("snake_case", case_converters::snake_case_filter);
    tera.register_filter("get_states_of_style", style_extractor::get_states_of_style);
    tera.register_filter("get_props_by_state", style_extractor::get_props_by_state);
    tera.register_filter("get_props", style_extractor::get_props);
    tera.register_filter("get_lv_state", style_extractor::get_lv_state);

    let mut ctx = tera::Context::new();
    ctx.insert("stylesheets", &stylesheets);
    if let Some(namespace) = namespace {
        ctx.insert("namespace", &namespace);
    }

    gen_file("styles.h.j2", "styles.h", &tera, &ctx)?;
    gen_file("stylesheets.h.j2", "stylesheets.h", &tera, &ctx)?;
    gen_file("stylesheets.cpp.j2", "stylesheets.cpp", &tera, &ctx)?;

    for stylesheet in stylesheets {
        ctx.insert("stylesheet", &stylesheet);
        let stylesheet_h_filename = format!("stylesheet_{}.h", stylesheet.name);
        let stylesheet_cpp_filename = format!("stylesheet_{}.cpp", stylesheet.name);
        gen_file("stylesheet.h.j2", &stylesheet_h_filename, &tera, &ctx)?;
        gen_file("stylesheet.cpp.j2", &stylesheet_cpp_filename, &tera, &ctx)?;
        ctx.remove("stylesheet");
    }


    Ok(())
}

fn gen_file(input: &str, output: &str, tera: &tera::Tera, ctx: &tera::Context) -> Result<(), String> {
    let gen_code = match tera.render(input, &ctx) {
        Ok(code) => code,
        Err(e) => return Err(format!("Unable to format C style files: {}", e)),
    };

    let mut file = match fs::File::create(&output) {
        Ok(f) => f,
        Err(e) => return Err(format!("Unable to create output file: {}", e)),
    };

    match file.write_all(gen_code.as_bytes()) {
        Ok(_) => {
            log::debug!("finished writing to {}", output);
            Ok(())
        },
        Err(e) => Err(format!("Unable to write to output file: {}", e)),
    }
}
