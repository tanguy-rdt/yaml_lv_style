use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use assert_cmd::cargo;
use assert_cmd::prelude::*;
use pretty_assertions::assert_eq as pretty_assert_eq;
use similar::TextDiff;
use tree_sitter::Parser;

pub fn yaml_lv_style(lang: &str, yaml_paths: &[&Path], output_dir: &Path) {
    Command::new(cargo::cargo_bin!("yaml_lv_style"))
        .arg("-f")
        .arg("google")
        .arg("-l")
        .arg(lang)
        .arg("-i")
        .args(yaml_paths)
        .arg("-o")
        .arg(output_dir)
        .assert()
        .success();
}

fn parse_c_code(code: &str) -> String {
    let mut parser = Parser::new();

    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("Error loading C grammar");

    let tree = parser.parse(code, None).expect("Failed to parse code");

    tree.root_node().to_sexp()
}

fn parse_cpp_code(code: &str) -> String {
    let mut parser = Parser::new();

    parser
        .set_language(&tree_sitter_cpp::LANGUAGE.into())
        .expect("Error loading C++ grammar");

    let tree = parser.parse(code, None).expect("Failed to parse code");

    tree.root_node().to_sexp()
}

pub fn compare_directory(expected_dir: &Path, generated_dir: &Path) {
    let mut paths_map: HashMap<PathBuf, PathBuf> = HashMap::new();

    for entry in walkdir::WalkDir::new(generated_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let gen_path = entry.path();
        if gen_path.is_file() {
            let ext = gen_path.extension().and_then(|s| s.to_str()).unwrap_or("");
            if ext == "cpp" || ext == "c" || ext == "h" {
                let relative = gen_path.strip_prefix(generated_dir).unwrap();
                let exp_path = expected_dir.join(relative);
                paths_map.insert(exp_path, gen_path.to_path_buf());
            }
        }
    }

    compare_files(&paths_map);
}

pub fn compare_files(path: &HashMap<PathBuf, PathBuf>) {
    for (expected_path, generated_path) in path {
        let generated_content = fs::read_to_string(generated_path)
            .unwrap_or_else(|_| panic!("Unable to read the generated file: {:?}", generated_path));

        if !expected_path.exists() {
            if let Some(parent) = expected_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(expected_path, &generated_content).unwrap();
        }

        let expected_content =
            fs::read_to_string(expected_path).expect("Unable to read the expected file");

        let ext = generated_path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("");
        let parser_fn = match ext {
            "cpp" | "hpp" => parse_cpp_code,
            _ => parse_c_code,
        };

        let generated_ast = parser_fn(&generated_content);
        let expected_ast = parser_fn(&expected_content);

        if generated_ast != expected_ast || generated_content != expected_content {
            let diff = TextDiff::from_lines(&expected_content, &generated_content);

            let patch = diff
                .unified_diff()
                .header(
                    &expected_path.to_string_lossy(),
                    &generated_path.to_string_lossy(),
                )
                .to_string();

            let mut diff_path = expected_path.clone();
            diff_path.set_extension("diff");

            fs::write(&diff_path, patch).expect("Unable to write the .diff file");

            pretty_assert_eq!(
                generated_content,
                expected_content,
                "The content of the file {:?} has diverged. A Unified Diff file has been generated in {:?}",
                generated_path.file_name().unwrap(),
                diff_path
            );
        }
    }
}
