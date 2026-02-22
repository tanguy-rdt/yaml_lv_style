use std::fs;
use std::path::PathBuf;
use std::process::Command;

use tera::Tera;

use crate::generator::filters::filters;

use super::cpp_stylesheets_context::{CppFileContext, CppStyleSheetsContext};

pub struct CppLang {
    tera: Tera,
    pub sources: Vec<PathBuf>,
    pub headers: Vec<PathBuf>,
}

impl CppLang {
    pub fn new() -> Result<Self, String> {
        let mut tera = Tera::default();

        filters::apply_filters(&mut tera);

        tera.add_raw_template("styles.h.tera", include_str!("templates/styles.h.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'styles.h.tera': {}", e))?;
        tera.add_raw_template("stylesheet.h.tera", include_str!("templates/stylesheet.h.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheet.h.tera': {}", e))?;
        tera.add_raw_template("stylesheet.cpp.tera", include_str!("templates/stylesheet.cpp.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheet.cpp.tera': {}", e))?;
        tera.add_raw_template("stylesheets.h.tera", include_str!("templates/stylesheets.h.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheets.h.tera': {}", e))?;
        tera.add_raw_template("stylesheets.cpp.tera", include_str!("templates/stylesheets.cpp.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheets.cpp.tera': {}", e))?;
        
        Ok(CppLang {
            tera,
            sources: Vec::new(),
            headers: Vec::new(),
        })
    }

    pub fn generate(&mut self, context: &CppStyleSheetsContext) -> Result<(), String> {
        let path = self.generate_file(&context.styles_name)?;
        self.headers.push(path);

        let path = self.generate_file(&context.stylesheets_helper.source)?;
        self.sources.push(path);
        let path = self.generate_file(&context.stylesheets_helper.header)?;
        self.headers.push(path);

        for stylesheet in &context.stylesheets {
            let path =   self.generate_file(&stylesheet.source)?;
            self.sources.push(path);
            let path =   self.generate_file(&stylesheet.header)?;
            self.headers.push(path);
        }
        Ok(())
    }

    pub fn format(&self, format: Option<&str>) -> Result<(), String> {
        if let Some(format_style) = &format {
            let status = Command::new("clang-format")
                .arg("-i")
                .arg(format!("-style={}", format_style))
                .args(self.headers.iter().chain(self.sources.iter())) // On passe tout d'un coup
                .status();

            return match status {
                Ok(s) if s.success() => Ok(()),
                Ok(_) => Err("clang-format failed to format".to_string()),
                Err(_) => Err("clang-format not found in PATH".to_string())
            };
        }

        Ok(())
    }

    fn generate_file(&mut self, ctx: &CppFileContext) -> Result<PathBuf, String> {
        let output_dir = ctx.path.parent()
            .ok_or_else(|| format!("Unable to determine the parent folder for '{}'", ctx.path.display()))?;

        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to created directory '{}': {}", output_dir.display(), e))?;

        let res = self.tera.render(&ctx.tera_template, &ctx.tera_context)
            .map_err(|e| format!("{}", e))?;

        fs::write(&ctx.path, res)
            .map_err(|e| format!("Failed to write to '{}': {}", ctx.path.display(), e))?;

        Ok(ctx.path.clone())
    }
}