use std::path::Path;

use super::file::Component;
use super::file::FileCtx;
use super::path_builder::PathBuilder;
use super::tera_ext::TeraContextExt;

use crate::serde_stylesheet::CanonicalStyleSheet;

pub struct BaseCtx {
    pub styles_name: FileCtx,
    pub stylesheets_helper: Component,
    pub stylesheets: Vec<Component>,
    #[cfg(feature = "macros")]
    pub macros: FileCtx,
}

impl BaseCtx {
    pub fn from_stylesheets(
        stylesheets: &[CanonicalStyleSheet],
        output_dir: &Path,
    ) -> Result<Self, String> {
        let paths = PathBuilder::new(output_dir);

        Ok(Self {
            styles_name: Self::make_styles_name_ctx(stylesheets, &paths)?,
            stylesheets_helper: Self::make_stylesheets_helper_ctx(stylesheets, &paths)?,
            stylesheets: Self::make_stylesheet_ctx(stylesheets, &paths)?,
            #[cfg(feature = "macros")]
            macros: Self::make_macros_ctx(stylesheets, &paths)?,
        })
    }

    fn make_styles_name_ctx(
        stylesheets: &[CanonicalStyleSheet],
        paths: &PathBuilder,
    ) -> Result<FileCtx, String> {
        let mut ctx = tera::Context::new();
        ctx.insert_or_err("stylesheets", &stylesheets)?;

        Ok(FileCtx {
            tera_template: String::from("styles_header"),
            tera_context: ctx,
            path: paths.styles_header_path(),
        })
    }

    fn make_stylesheets_helper_ctx(
        stylesheets: &[CanonicalStyleSheet],
        paths: &PathBuilder,
    ) -> Result<Component, String> {
        let mut ctx = tera::Context::new();
        ctx.insert_or_err("stylesheets", &stylesheets)?;
        ctx.insert_or_err("h_stylesheets_include_dir_path", &paths.folder())?;
        ctx.insert_or_err(
            "h_stylesheets_include_path",
            &paths.h_stylesheets_include_path(),
        )?;
        ctx.insert_or_err("h_styles_include_path", &paths.h_styles_include_path())?;

        Ok(Component {
            source: FileCtx {
                tera_template: String::from("stylesheets_source"),
                tera_context: ctx.clone(),
                path: paths.stylesheets_source_path(),
            },
            header: FileCtx {
                tera_template: String::from("stylesheets_header"),
                tera_context: ctx,
                path: paths.stylesheets_header_path(),
            },
        })
    }

    fn make_stylesheet_ctx(
        stylesheets: &[CanonicalStyleSheet],
        paths: &PathBuilder,
    ) -> Result<Vec<Component>, String> {
        stylesheets
            .iter()
            .map(|stylesheet| {
                let name = stylesheet.get_name();

                let mut ctx = tera::Context::new();
                ctx.insert_or_err("stylesheet", &stylesheet)?;
                ctx.insert_or_err(
                    "h_stylesheet_include_path",
                    &paths.h_stylesheet_include_path(name),
                )?;
                ctx.insert_or_err("h_styles_include_path", &paths.h_styles_include_path())?;

                Ok(Component {
                    source: FileCtx {
                        tera_template: String::from("stylesheet_source"),
                        tera_context: ctx.clone(),
                        path: paths.stylesheet_source_path(name),
                    },
                    header: FileCtx {
                        tera_template: String::from("stylesheet_header"),
                        tera_context: ctx,
                        path: paths.stylesheet_header_path(name),
                    },
                })
            })
            .collect()
    }

    #[cfg(feature = "macros")]
    fn make_macros_ctx(
        stylesheets: &[CanonicalStyleSheet],
        paths: &PathBuilder,
    ) -> Result<FileCtx, String> {
        let mut ctx = tera::Context::new();
        ctx.insert_or_err("stylesheets", &stylesheets)?;
        ctx.insert_or_err("h_stylesheets_include_dir_path", &paths.folder())?;
        ctx.insert_or_err(
            "h_stylesheets_include_path",
            &paths.h_stylesheets_include_path(),
        )?;
        ctx.insert_or_err("h_styles_include_path", &paths.h_styles_include_path())?;

        Ok(FileCtx {
            tera_template: String::from("macros"),
            tera_context: ctx,
            path: paths.macros_header_path(),
        })
    }

    pub fn insert_namespace(&mut self, namespace: &str) {
        self.styles_name.tera_context.insert("namespace", namespace);
        self.stylesheets_helper
            .source
            .tera_context
            .insert("namespace", namespace);
        self.stylesheets_helper
            .header
            .tera_context
            .insert("namespace", namespace);
        for stylesheet in &mut self.stylesheets {
            stylesheet
                .source
                .tera_context
                .insert("namespace", namespace);
            stylesheet
                .header
                .tera_context
                .insert("namespace", namespace);
        }
        #[cfg(feature = "macros")]
        self.macros.tera_context.insert("namespace", namespace);
    }
}
