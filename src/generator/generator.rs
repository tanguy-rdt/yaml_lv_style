use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use super::filters::case_converters;
use super::filters::style_extractor;
use super::filters::style_info;

use crate::stylesheet::stylesheet::StyleSheet;

pub fn generate(stylesheets: &Vec<StyleSheet>, output_dir: &Path, namespace: &Option<String>, format_style: &Option<String>, print_gen_file_path: bool) -> Result<(), String> {
    let output_folder_name = output_dir.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(".");

    let include_styles_header_dir = format!("{}/styles", output_folder_name);
    let output_styles_header_dir = output_dir.join(format!("styles/include/{}/styles", output_folder_name));
    fs::create_dir_all(&output_styles_header_dir)
        .map_err(|e| format!("Failed to created directory '{}': {}", output_dir.display(), e))?;

    let include_stylesheets_header_dir = format!("{}/stylesheets", output_folder_name);
    let output_stylesheets_header_dir = output_dir.join(format!("stylesheets/include/{}/stylesheets", output_folder_name));
    fs::create_dir_all(&output_stylesheets_header_dir)
        .map_err(|e| format!("Failed to created directory '{}': {}", output_dir.display(), e))?;

    let output_stylesheets_source_dir = output_dir.join("stylesheets/src/");
    fs::create_dir_all(&output_stylesheets_source_dir)
        .map_err(|e| format!("Failed to created directory '{}': {}", output_dir.display(), e))?;

    let mut tera = tera::Tera::default();

    tera.add_raw_template("styles.h.j2", include_str!("templates/styles.h.j2"))
        .map_err(|e| format!("An error occurred while loading the Tera template 'styles.h.j2': {}", e))?;
    tera.add_raw_template("stylesheet.h.j2", include_str!("templates/stylesheet.h.j2"))
        .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheet.h.j2': {}", e))?;
    tera.add_raw_template("stylesheet.cpp.j2", include_str!("templates/stylesheet.cpp.j2"))
        .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheet.cpp.j2': {}", e))?;
    tera.add_raw_template("stylesheets.h.j2", include_str!("templates/stylesheets.h.j2"))
        .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheets.h.j2': {}", e))?;
    tera.add_raw_template("stylesheets.cpp.j2", include_str!("templates/stylesheets.cpp.j2"))
        .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheets.cpp.j2': {}", e))?;

    tera.register_filter("pascal_case", case_converters::pascal_case_filter);
    tera.register_filter("screaming_snake_case", case_converters::screaming_snake_case_filter);
    tera.register_filter("snake_case", case_converters::snake_case_filter);
    tera.register_filter("get_states_of_style", style_extractor::get_states_of_style);
    tera.register_filter("get_props_by_states_sorted", style_extractor::get_props_by_states_sorted);
    tera.register_filter("get_props_sorted", style_extractor::get_props_sorted);
    tera.register_filter("get_lv_state", style_extractor::get_lv_state);
    tera.register_filter("has_const_style", style_info::has_const_style);
    tera.register_filter("has_dyn_style", style_info::has_dyn_style);

    let mut ctx = tera::Context::new();
    ctx.insert("stylesheets", &stylesheets);
    ctx.insert("include_styles_header_dir", &include_styles_header_dir);
    ctx.insert("include_stylesheets_header_dir", &include_stylesheets_header_dir);
    if let Some(namespace) = namespace {
        ctx.insert("namespace", &namespace);
    }

    let files_to_render = HashMap::from([
        ("styles.h.j2", output_styles_header_dir.join("styles.h")),
        ("stylesheets.h.j2", output_stylesheets_header_dir.join("stylesheets.h")),
        ("stylesheets.cpp.j2", output_stylesheets_source_dir.join("stylesheets.cpp")),
    ]);
    create_lv_stylesheet(&files_to_render, format_style, &ctx, &tera, print_gen_file_path)?;

    for stylesheet in stylesheets {
        ctx.insert("stylesheet", &stylesheet);

        let files_to_render = HashMap::from([
            ("stylesheet.h.j2", output_stylesheets_header_dir.join(format!("stylesheet_{}.h", stylesheet.name))),
            ("stylesheet.cpp.j2", output_stylesheets_source_dir.join(format!("stylesheet_{}.cpp", stylesheet.name))),
        ]);
        create_lv_stylesheet(&files_to_render, format_style, &ctx, &tera, print_gen_file_path)?;

        ctx.remove("stylesheet");
    }

    Ok(())
}

fn create_lv_stylesheet(files: &HashMap<&str, PathBuf>, format_style: &Option<String>, ctx: &tera::Context, tera: &tera::Tera, print_gen_file_path: bool) -> Result<(), String> {
    for (template, to_generate) in files {
        let styles_renderd = render(template.as_ref(), &ctx, &tera)?;
        create(styles_renderd, &to_generate)?;
        format(&to_generate, format_style)?;

        if print_gen_file_path {
            println!("{}", to_generate.display());
        }
    }

    Ok(())
}

fn render(input_template: &Path, ctx: &tera::Context, tera: &tera::Tera) -> Result<String, String> {
    let template_name = input_template
        .to_str()
        .ok_or_else(|| format!("Invalid template path: {}", input_template.display()))?;

    tera.render(template_name, ctx)
        .map_err(|e| format!("Unable to render template '{}': {}", template_name, e))
}

fn create(rendered_code: String, output: &Path) -> Result<(), String> {
    fs::write(output, rendered_code)
        .map_err(|e| format!("Failed to write to '{}': {}", output.display(), e))
}
fn format(created_file: &Path, format_code: &Option<String>) -> Result<(), String> {
    if let Some(format_style) = format_code {
        let status = Command::new("clang-format")
            .arg("-i")
            .arg(format!("-style={}", format_style))
            .arg(created_file)
            .status();

        match status {
            Ok(s) if s.success() => return Ok(()),
            Ok(_) => return Err(format!("clang-format failed to format {}", created_file.display())),
            Err(_) => return Err(format!("clang-format not found in PATH, skipping formatting for {}", created_file.display()))
        };
    }

    Ok(())
}