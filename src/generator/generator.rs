use std::path::PathBuf;

use super::languages::cpp_lang::cpp_lang::CppLang;
use super::languages::cpp_lang::cpp_stylesheets_context::CppStyleSheetsContext;

use crate::stylesheet::stylesheet::StyleSheet;


pub struct Generator {
    output_dir: PathBuf,
    format: Option<String>,
    cpp_lang: CppLang
}

impl Generator {
    pub fn new(output_dir: PathBuf, format: Option<String>) -> Self {
        Generator {
            output_dir,
            format,
            cpp_lang: CppLang::new().unwrap()
        }
    }

    pub fn generate_cpp(&mut self, namespace: Option<&str>, stylesheets: &[StyleSheet]) -> Result<(), String> {
        let cpp_ctx = CppStyleSheetsContext::from_stylesheets(stylesheets, namespace, &self.output_dir);

        // let mut cpp_lang = CppLang::new()
        //     .map_err(|e| format!("Failed to create CppLang: {}", e))?;
        self.cpp_lang.generate(&cpp_ctx)
            .map_err(|e| format!("Failed to generate Cpp files: {}", e))?;
        self.cpp_lang.format(self.format.as_deref())
    }

    pub fn print_generated_files_path(&self) {
        for file in self.cpp_lang.headers.iter().chain(self.cpp_lang.sources.iter()) {
            println!("{}", file.display());
        }
    }
}