mod c_generation_ctx;
mod cpp_generation_ctx;

use std::path::{Path, PathBuf};

use crate::serde_stylesheet::StyleSheet;

pub use c_generation_ctx::CGenerationCtx;
pub use cpp_generation_ctx::CppGenerationCtx;

pub struct FileCtx {
    pub tera_template: String,
    pub tera_context: tera::Context,
    pub path: PathBuf,
}

pub struct Component {
    pub source: FileCtx,
    pub header: FileCtx,
}

pub struct GenerationCtx {
    pub styles_name: FileCtx,
    pub stylesheets_helper: Component,
    pub stylesheets: Vec<Component>,
}

impl GenerationCtx {
    pub fn from_stylesheets(stylesheet: &[StyleSheet], output_dir: &Path) -> Result<Self, String> {
        Ok(Self {
            styles_name: Self::make_styles_name_ctx(stylesheet, output_dir)?,
            stylesheets_helper: Self::make_stylesheets_helper_ctx(stylesheet, output_dir)?,
            stylesheets: Self::make_stylesheet_ctx(stylesheet, output_dir)?,
        })
    }

    fn make_styles_name_ctx(
        stylesheets: &[StyleSheet],
        output_dir: &Path,
    ) -> Result<FileCtx, String> {
        let output_folder_name = output_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");

        let header_path =
            output_dir.join(format!("styles/include/{}/styles.h", output_folder_name));

        let mut tera_ctx = tera::Context::new();
        tera_ctx
            .try_insert("stylesheets", &stylesheets)
            .map_err(|e| format!("{e}"))?;

        Ok(FileCtx {
            tera_template: String::from("styles_header"),
            tera_context: tera_ctx,
            path: header_path,
        })
    }

    fn make_stylesheets_helper_ctx(
        stylesheets: &[StyleSheet],
        output_dir: &Path,
    ) -> Result<Component, String> {
        let output_folder_name = output_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");

        let source_path = output_dir.join("stylesheets/src/stylesheets");

        let header_path = output_dir.join(format!(
            "stylesheets/include/{}/stylesheets.h",
            output_folder_name
        ));
        let h_stylesheets_include_dir_path = output_folder_name.to_string();
        let h_stylesheets_include_path = format!("{}/stylesheets.h", output_folder_name);
        let h_styles_include_path = format!("{}/styles.h", output_folder_name);

        let mut tera_ctx = tera::Context::new();
        tera_ctx
            .try_insert("stylesheets", &stylesheets)
            .map_err(|e| format!("{e}"))?;
        tera_ctx
            .try_insert(
                "h_stylesheets_include_dir_path",
                &h_stylesheets_include_dir_path,
            )
            .map_err(|e| format!("{e}"))?;
        tera_ctx
            .try_insert("h_stylesheets_include_path", &h_stylesheets_include_path)
            .map_err(|e| format!("{e}"))?;
        tera_ctx
            .try_insert("h_styles_include_path", &h_styles_include_path)
            .map_err(|e| format!("{e}"))?;

        let source = FileCtx {
            tera_template: String::from("stylesheets_source"),
            tera_context: tera_ctx.clone(),
            path: source_path,
        };

        let header = FileCtx {
            tera_template: String::from("stylesheets_header"),
            tera_context: tera_ctx,
            path: header_path,
        };

        Ok(Component { source, header })
    }

    fn make_stylesheet_ctx(
        stylesheets: &[StyleSheet],
        output_dir: &Path,
    ) -> Result<Vec<Component>, String> {
        let mut stylesheet_ctx = Vec::new();

        let output_folder_name = output_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");

        for stylesheet in stylesheets {
            let source_path =
                output_dir.join(format!("stylesheets/src/stylesheet_{}", stylesheet.name));

            let header_path = output_dir.join(format!(
                "stylesheets/include/{}/stylesheet_{}.h",
                output_folder_name, stylesheet.name
            ));
            let h_stylesheet_include_path =
                format!("{}/stylesheet_{}.h", output_folder_name, stylesheet.name);
            let h_styles_include_path = format!("{}/styles.h", output_folder_name);

            let mut tera_ctx = tera::Context::new();
            tera_ctx
                .try_insert("stylesheet", &stylesheet)
                .map_err(|e| format!("{e}"))?;
            tera_ctx
                .try_insert("h_stylesheet_include_path", &h_stylesheet_include_path)
                .map_err(|e| format!("{e}"))?;
            tera_ctx
                .try_insert("h_styles_include_path", &h_styles_include_path)
                .map_err(|e| format!("{e}"))?;

            let source = FileCtx {
                tera_template: String::from("stylesheet_source"),
                tera_context: tera_ctx.clone(),
                path: source_path,
            };

            let header = FileCtx {
                tera_template: String::from("stylesheet_header"),
                tera_context: tera_ctx,
                path: header_path,
            };

            let component = Component { source, header };

            stylesheet_ctx.push(component);
        }

        Ok(stylesheet_ctx)
    }
}
