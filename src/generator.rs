mod code_style;
mod format;
mod generation_ctx;
mod lang;
mod tera_filters;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub use format::ClangFormatStyle;
pub use lang::Language;

use generation_ctx::BaseCtx;
use generation_ctx::C;
use generation_ctx::Component;
use generation_ctx::Cpp;
use generation_ctx::FileCtx;
use generation_ctx::LanguageCtx;
use generation_ctx::TeraExt;

use crate::errors::Error;
use crate::errors::Result;
use crate::serde_stylesheet::CanonicalStyleSheet;

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

    pub fn generate_cpp(
        &mut self,
        namespace: Option<&str>,
        stylesheets: &[CanonicalStyleSheet],
    ) -> Result<()> {
        self.generate::<Cpp>(stylesheets, namespace)
    }

    pub fn generate_c(&mut self, stylesheets: &[CanonicalStyleSheet]) -> Result<()> {
        self.generate::<C>(stylesheets, None)
    }

    fn generate<L: LanguageCtx>(
        &mut self,
        stylesheets: &[CanonicalStyleSheet],
        namespace: Option<&str>,
    ) -> Result<()> {
        let mut ctx = BaseCtx::from_stylesheets(stylesheets, &self.output_dir)
            .map_err(|e| Error::Generation(Box::new(Error::Other(e))))?;

        let tera = Self::build_tera::<L>(&mut ctx, namespace)
            .map_err(|e| Error::Generation(Box::new(Error::Other(e.to_string()))))?;

        self.render_ctx(&tera, &ctx)
            .map_err(|e| Error::Generation(Box::from(e)))
    }

    fn build_tera<L: LanguageCtx>(
        base: &mut BaseCtx,
        namespace: Option<&str>,
    ) -> std::result::Result<tera::Tera, String> {
        let mut tera = tera::Tera::default();
        tera_filters::apply_filters(&mut tera);

        for (name, content) in L::templates() {
            tera.add_raw_template_or_err(name, content)?;
        }

        L::setup(base, namespace);

        Ok(tera)
    }

    fn render_ctx(&mut self, tera: &tera::Tera, ctx: &BaseCtx) -> Result<()> {
        #[cfg(feature = "macros")]
        self.headers.push(self.render_file(tera, &ctx.macros)?);
        self.headers.push(self.render_file(tera, &ctx.styles_name)?);
        self.render_component(tera, &ctx.stylesheets_helper)?;
        for stylesheet in &ctx.stylesheets {
            self.render_component(tera, stylesheet)?;
        }

        self.format()
    }

    fn render_component(&mut self, tera: &tera::Tera, component: &Component) -> Result<()> {
        self.sources
            .push(self.render_file(tera, &component.source)?);
        self.headers
            .push(self.render_file(tera, &component.header)?);
        Ok(())
    }

    fn render_file(&self, tera: &tera::Tera, ctx: &FileCtx) -> Result<PathBuf> {
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

    fn format(&self) -> Result<()> {
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

    pub fn get_generated_headers_path(&self) -> Vec<PathBuf> {
        self.headers.clone()
    }

    pub fn get_generated_sources_path(&self) -> Vec<PathBuf> {
        self.sources.clone()
    }
}
