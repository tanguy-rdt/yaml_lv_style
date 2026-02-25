mod format;
mod lang;
mod stylesheet_ctx;
mod tera_filters;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub use format::ClangFormatStyle;
pub use lang::Language;
use stylesheet_ctx::CStyleSheetsContext;
use stylesheet_ctx::CppStyleSheetsContext;
use stylesheet_ctx::FileContext;
use stylesheet_ctx::StyleSheetsContext;

use crate::serde_stylesheet::StyleSheet;

pub struct Generator {
    output_dir: PathBuf,
    ctx: Option<StyleSheetsContext>,
    format: Option<ClangFormatStyle>,
    sources: Vec<PathBuf>,
    headers: Vec<PathBuf>,
}

impl Generator {
    pub fn new(output_dir: PathBuf, format: Option<ClangFormatStyle>) -> Self {
        Generator {
            output_dir,
            ctx: None,
            format,
            sources: Vec::new(),
            headers: Vec::new(),
        }
    }

    pub fn generate_c(&mut self, stylesheets: &[StyleSheet]) -> Result<(), String> {
        let mut ctx = StyleSheetsContext::from_stylesheets(stylesheets, &self.output_dir);
        let c_ctx = CStyleSheetsContext::from(&mut ctx)
            .map_err(|e| format!("Failed to create C context: {}", e))?;

        self.render_ctx(&c_ctx.tera, c_ctx.base)?;

        self.ctx = Some(ctx);

        Ok(())
    }

    pub fn generate_cpp(
        &mut self,
        namespace: Option<&str>,
        stylesheets: &[StyleSheet],
    ) -> Result<(), String> {
        let mut ctx = StyleSheetsContext::from_stylesheets(stylesheets, &self.output_dir);
        let cpp_ctx = CppStyleSheetsContext::from(&mut ctx, namespace)
            .map_err(|e| format!("Failed to create C++ context: {}", e))?;

        self.render_ctx(&cpp_ctx.tera, cpp_ctx.base)?;

        self.ctx = Some(ctx);

        Ok(())
    }

    fn render_ctx(&mut self, tera: &tera::Tera, ctx: &StyleSheetsContext) -> Result<(), String> {
        let path = self.render_file(tera, &ctx.styles_name)?;
        self.headers.push(path);

        let path = self.render_file(tera, &ctx.stylesheets_helper.source)?;
        self.sources.push(path);
        let path = self.render_file(tera, &ctx.stylesheets_helper.header)?;
        self.headers.push(path);

        for stylesheet in &ctx.stylesheets {
            let path = self.render_file(tera, &stylesheet.source)?;
            self.sources.push(path);
            let path = self.render_file(tera, &stylesheet.header)?;
            self.headers.push(path);
        }

        self.format()
    }

    fn render_file(&mut self, tera: &tera::Tera, ctx: &FileContext) -> Result<PathBuf, String> {
        let output_dir = ctx.path.parent().ok_or_else(|| {
            format!(
                "Unable to determine the parent folder for '{}'",
                ctx.path.display()
            )
        })?;

        fs::create_dir_all(output_dir).map_err(|e| {
            format!(
                "Failed to created directory '{}': {}",
                output_dir.display(),
                e
            )
        })?;

        let res = tera
            .render(&ctx.tera_template, &ctx.tera_context)
            .map_err(|e| format!("{}", e))?;

        fs::write(&ctx.path, res)
            .map_err(|e| format!("Failed to write to '{}': {}", ctx.path.display(), e))?;

        Ok(ctx.path.clone())
    }

    fn format(&self) -> Result<(), String> {
        if let Some(format_style) = &self.format {
            let status = Command::new("clang-format")
                .arg("-i")
                .arg(format!("-style={}", format_style.to_clang_preset()))
                .args(self.headers.iter().chain(self.sources.iter()))
                .status();

            return match status {
                Ok(s) if s.success() => Ok(()),
                Ok(_) => Err("clang-format failed to format".to_string()),
                Err(_) => Err("clang-format not found in PATH".to_string()),
            };
        }

        Ok(())
    }

    pub fn print_generated_files_path(&self) {
        for file in self.headers.iter().chain(self.sources.iter()) {
            println!("{}", file.display());
        }
    }
}
