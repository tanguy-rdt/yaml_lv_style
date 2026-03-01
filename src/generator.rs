mod format;
mod generation_ctx;
mod lang;
mod tera_filters;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub use format::ClangFormatStyle;
pub use lang::Language;

use generation_ctx::CGenerationCtx;
use generation_ctx::CppGenerationCtx;
use generation_ctx::FileCtx;
use generation_ctx::GenerationCtx;

use crate::errors::Error;
use crate::errors::YamlLvStyleResult;
use crate::serde_stylesheet::StyleSheet;

pub struct Generator {
    output_dir: PathBuf,
    format: Option<ClangFormatStyle>,
    sources: Vec<PathBuf>,
    headers: Vec<PathBuf>,
}

impl Generator {
    pub fn new(output_dir: PathBuf, format: Option<ClangFormatStyle>) -> Self {
        Generator {
            output_dir,
            format,
            sources: Vec::new(),
            headers: Vec::new(),
        }
    }

    pub fn generate_c(&mut self, stylesheets: &[StyleSheet]) -> YamlLvStyleResult<()> {
        let mut ctx = GenerationCtx::from_stylesheets(stylesheets, &self.output_dir)
            .map_err(|e| Error::Generation(Box::new(Error::Other(e))))?;
        let c_ctx = CGenerationCtx::from(&mut ctx)
            .map_err(|e| Error::Generation(Box::new(Error::Other(e))))?;

        self.render_ctx(&c_ctx.tera, &ctx)
            .map_err(|e| Error::Generation(Box::from(e)))?;

        Ok(())
    }

    pub fn generate_cpp(
        &mut self,
        namespace: Option<&str>,
        stylesheets: &[StyleSheet],
    ) -> YamlLvStyleResult<()> {
        let mut ctx = GenerationCtx::from_stylesheets(stylesheets, &self.output_dir)
            .map_err(|e| Error::Generation(Box::new(Error::Other(e))))?;
        let cpp_ctx = CppGenerationCtx::from(&mut ctx, namespace)
            .map_err(|e| Error::Generation(Box::new(Error::Other(e))))?;

        self.render_ctx(&cpp_ctx.tera, &ctx)
            .map_err(|e| Error::Generation(Box::from(e)))?;

        Ok(())
    }

    fn render_ctx(&mut self, tera: &tera::Tera, ctx: &GenerationCtx) -> YamlLvStyleResult<()> {
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

    fn render_file(&self, tera: &tera::Tera, ctx: &FileCtx) -> YamlLvStyleResult<PathBuf> {
        let output_dir = ctx
            .path
            .parent()
            .ok_or_else(|| Error::IoKind(std::io::ErrorKind::InvalidInput, ctx.path.clone()))?;

        fs::create_dir_all(output_dir).map_err(|e| Error::Io(e, output_dir.to_path_buf()))?;

        let res = tera
            .render(&ctx.tera_template, &ctx.tera_context)
            .map_err(|e| Error::Render(e.to_string()))?;

        fs::write(&ctx.path, res).map_err(|e| Error::Io(e, ctx.path.clone()))?;

        Ok(ctx.path.clone())
    }

    fn format(&self) -> YamlLvStyleResult<()> {
        if let Some(format_style) = &self.format {
            let status = Command::new("clang-format")
                .arg("-i")
                .arg(format!("-style={}", format_style.to_clang_preset()))
                .args(self.headers.iter().chain(self.sources.iter()))
                .status();

            return match status {
                Ok(s) if s.success() => Ok(()),
                Ok(_) => Err(Error::Format("clang-format failed to format".to_string())),
                Err(_) => Err(Error::Format("clang-format not found in PATH".to_string())),
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
