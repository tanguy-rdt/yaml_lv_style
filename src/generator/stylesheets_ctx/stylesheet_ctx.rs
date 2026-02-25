use std::path::{Path, PathBuf};

use crate::stylesheet::stylesheet::StyleSheet;

pub struct FileContext {
    pub tera_template: String,
    pub tera_context: tera::Context,
    pub path: PathBuf,
}

pub struct Component {
    pub source: FileContext,
    pub header: FileContext,
}

pub struct StyleSheetsContext {
    pub styles_name: FileContext,
    pub stylesheets_helper: Component,
    pub stylesheets: Vec<Component>,
}

impl StyleSheetsContext {
    pub fn from_stylesheets(stylesheet: &[StyleSheet], output_dir: &Path) -> Self {
        Self {
            styles_name: Self::make_styles_name_ctx(stylesheet, output_dir),
            stylesheets_helper: Self::make_stylesheets_helper_ctx(stylesheet, output_dir),
            stylesheets: Self::make_stylesheet_ctx(stylesheet, output_dir),
        }
    }

    fn make_styles_name_ctx(stylesheets: &[StyleSheet], output_dir: &Path) -> FileContext {
        let output_folder_name = output_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");

        let header_path =
            output_dir.join(format!("styles/include/{}/styles.h", output_folder_name));

        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("stylesheets", &stylesheets);

        FileContext {
            tera_template: String::from("styles_header"),
            tera_context: tera_ctx,
            path: header_path,
        }
    }

    fn make_stylesheets_helper_ctx(stylesheets: &[StyleSheet], output_dir: &Path) -> Component {
        let output_folder_name = output_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");

        let source_path = output_dir.join("stylesheets/src/stylesheets");

        let header_path = output_dir.join(format!(
            "stylesheets/include/{}/stylesheets.h",
            output_folder_name
        ));
        let h_stylesheets_include_dir_path = format!("{}", output_folder_name);
        let h_stylesheets_include_path = format!("{}/stylesheets.h", output_folder_name);
        let h_styles_include_path = format!("{}/styles.h", output_folder_name);

        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("stylesheets", &stylesheets);
        tera_ctx.insert(
            "h_stylesheets_include_dir_path",
            &h_stylesheets_include_dir_path,
        );
        tera_ctx.insert("h_stylesheets_include_path", &h_stylesheets_include_path);
        tera_ctx.insert("h_styles_include_path", &h_styles_include_path);

        let source = FileContext {
            tera_template: String::from("stylesheets_source"),
            tera_context: tera_ctx.clone(),
            path: source_path,
        };

        let header = FileContext {
            tera_template: String::from("stylesheets_header"),
            tera_context: tera_ctx,
            path: header_path,
        };

        Component { source, header }
    }

    fn make_stylesheet_ctx(stylesheets: &[StyleSheet], output_dir: &Path) -> Vec<Component> {
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
            tera_ctx.insert("stylesheet", &stylesheet);
            tera_ctx.insert("h_stylesheet_include_path", &h_stylesheet_include_path);
            tera_ctx.insert("h_styles_include_path", &h_styles_include_path);

            let source = FileContext {
                tera_template: String::from("stylesheet_source"),
                tera_context: tera_ctx.clone(),
                path: source_path,
            };

            let header = FileContext {
                tera_template: String::from("stylesheet_header"),
                tera_context: tera_ctx,
                path: header_path,
            };

            let component = Component { source, header };

            stylesheet_ctx.push(component);
        }

        stylesheet_ctx
    }
}
