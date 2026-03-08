use std::path::{Path, PathBuf};

pub struct PathBuilder<'a> {
    output_dir: &'a Path,
    folder: &'a str,
}

impl<'a> PathBuilder<'a> {
    pub fn new(output_dir: &'a Path) -> Self {
        let folder = output_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(".");
        Self { output_dir, folder }
    }

    pub fn folder(&self) -> &str {
        self.folder
    }

    pub fn h_styles_include_path(&self) -> String {
        format!("{}/styles.h", self.folder)
    }

    pub fn h_stylesheets_include_path(&self) -> String {
        format!("{}/stylesheets.h", self.folder)
    }

    pub fn h_stylesheet_include_path(&self, name: &str) -> String {
        format!("{}/stylesheet_{}.h", self.folder, name)
    }

    pub fn styles_header_path(&self) -> PathBuf {
        self.output_dir
            .join(format!("styles/include/{}/styles.h", self.folder))
    }

    pub fn stylesheets_header_path(&self) -> PathBuf {
        self.output_dir
            .join(format!("stylesheets/include/{}/stylesheets.h", self.folder))
    }

    pub fn stylesheets_source_path(&self) -> PathBuf {
        self.output_dir.join("stylesheets/src/stylesheets")
    }

    pub fn stylesheet_header_path(&self, name: &str) -> PathBuf {
        self.output_dir.join(format!(
            "stylesheets/include/{}/stylesheet_{}.h",
            self.folder, name
        ))
    }

    pub fn stylesheet_source_path(&self, name: &str) -> PathBuf {
        self.output_dir
            .join(format!("stylesheets/src/stylesheet_{}", name))
    }

    #[cfg(feature = "macros")]
    pub fn macros_header_path(&self) -> PathBuf {
        self.output_dir.join(format!(
            "stylesheets/include/{}/stylesheets_macros.h",
            self.folder
        ))
    }
}
