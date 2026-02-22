use std::path::{Path, PathBuf};

use crate::stylesheet::stylesheet::StyleSheet;

pub struct CppFileContext {
    pub tera_template: String,
    pub tera_context: tera::Context,
    pub path: PathBuf,
}

pub struct CppComponent {
    pub source: CppFileContext,
    pub header: CppFileContext,
}

pub struct CppStyleSheetsContext {
    pub styles_name: CppFileContext,
    pub stylesheets_helper: CppComponent,
    pub stylesheets: Vec<CppComponent>,
}

impl CppStyleSheetsContext {
    pub fn from_stylesheets(stylesheet: &[StyleSheet], namespace: Option<&str>, output_dir: &Path) -> Self {
        Self {
            styles_name: Self::make_styles_name_ctx(stylesheet, namespace, output_dir),
            stylesheets_helper: Self::make_stylesheets_helper_ctx(stylesheet, namespace, output_dir),
            stylesheets: Self::make_stylesheet_ctx(stylesheet, namespace, output_dir),
        }
    }

    fn make_styles_name_ctx(stylesheets: &[StyleSheet], namespace: Option<&str>, output_dir: &Path) -> CppFileContext {
        let output_folder_name = output_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");

        let header_path = output_dir.join(format!("styles/include/{}/styles.h", output_folder_name));

        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("stylesheets", &stylesheets);
        if let Some(namespace) = namespace {
            tera_ctx.insert("namespace", &namespace);
        }

        CppFileContext {
            tera_template: String::from("styles.h.tera"),
            tera_context: tera_ctx,
            path: header_path,
        }
    }

    fn make_stylesheets_helper_ctx(stylesheets: &[StyleSheet], namespace: Option<&str>, output_dir: &Path) -> CppComponent {
        let output_folder_name = output_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");

        let source_path = output_dir.join("stylesheets/src/stylesheets.cpp");

        let header_path = output_dir.join(format!("stylesheets/include/{}/stylesheets.h", output_folder_name));
        let h_stylesheets_include_dir_path = format!("{}/stylesheets", output_folder_name);
        let h_stylesheets_include_path = format!("{}/stylesheets/stylesheets.h", output_folder_name);
        let h_styles_include_path = format!("{}/styles/styles.h", output_folder_name);

        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("stylesheets", &stylesheets);
        tera_ctx.insert("h_stylesheets_include_dir_path", &h_stylesheets_include_dir_path);
        tera_ctx.insert("h_stylesheets_include_path", &h_stylesheets_include_path);
        tera_ctx.insert("h_styles_include_path", &h_styles_include_path);
        if let Some(namespace) = namespace {
            tera_ctx.insert("namespace", &namespace);
        }

        let source = CppFileContext {
            tera_template: String::from("stylesheets.cpp.tera"),
            tera_context: tera_ctx.clone(),
            path: source_path
        };

        let header = CppFileContext {
            tera_template: String::from("stylesheets.h.tera"),
            tera_context: tera_ctx,
            path: header_path,
        };

        CppComponent { source, header }
    }

    fn make_stylesheet_ctx(stylesheets: &[StyleSheet], namespace: Option<&str>, output_dir: &Path) -> Vec<CppComponent> {
        let mut stylesheet_ctx = Vec::new();

        let output_folder_name = output_dir.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");

        for stylesheet in stylesheets {
            let source_path = output_dir.join(format!("stylesheets/src/stylesheet_{}.cpp", stylesheet.name));

            let header_path = output_dir.join(format!("stylesheets/include/{}/stylesheet_{}.h", output_folder_name, stylesheet.name));
            let h_stylesheet_include_path = format!("{}/stylesheets/stylesheet_{}.h", output_folder_name, stylesheet.name);
            let h_styles_include_path = format!("{}/styles/styles.h", output_folder_name);

            let mut tera_ctx = tera::Context::new();
            tera_ctx.insert("stylesheet", &stylesheet);
            tera_ctx.insert("h_stylesheet_include_path", &h_stylesheet_include_path);
            tera_ctx.insert("h_styles_include_path", &h_styles_include_path);
            if let Some(namespace) = namespace {
                tera_ctx.insert("namespace", &namespace);
            }

            let source = CppFileContext {
                tera_template: String::from("stylesheet.cpp.tera"),
                tera_context: tera_ctx.clone(),
                path: source_path
            };

            let header = CppFileContext {
                tera_template: String::from("stylesheet.h.tera"),
                tera_context: tera_ctx,
                path: header_path,
            };

            let cpp_component = CppComponent { source, header };

            stylesheet_ctx.push(cpp_component);
        }

        stylesheet_ctx
    }
}