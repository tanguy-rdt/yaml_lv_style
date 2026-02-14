use std::path::Path;
use std::path::PathBuf;

use tempfile::tempdir;

use super::helper::yaml_lv_style;
use super::helper::compare_directory;

#[macro_export]
macro_rules! assert_stylesheet {
    ($yaml:expr) => {
        let test_name = std::thread::current().name().unwrap_or("unknown_test").to_string();
        let full_function_name = test_name.split("::").last().unwrap_or(&test_name);
        let folder_name = full_function_name.strip_prefix("test_").unwrap_or(full_function_name);
        let expected_dir = format!("tests/expected_result/{}", folder_name);

        $crate::tools::assert::assert_stylesheet_with_expected($yaml, &expected_dir);
    };
}

#[macro_export]
macro_rules! assert_stylesheets {
    ($yamls:expr) => {
        let test_name = std::thread::current().name().unwrap_or("unknown_test").to_string();
        let full_function_name = test_name.split("::").last().unwrap_or(&test_name);
        let folder_name = full_function_name.strip_prefix("test_").unwrap_or(full_function_name);
        let expected_dir = format!("tests/expected_result/{}", folder_name);

        $crate::tools::assert::assert_stylesheets_with_expected($yamls, &expected_dir);
    };
}

pub fn assert_stylesheet_with_expected(yaml: &str, expected_dir_path: &str) {
    assert_stylesheets_with_expected(&[yaml], expected_dir_path);
}

pub fn assert_stylesheets_with_expected(yamls: &[&str], expected_dir_path: &str) {
    let tmp_dir = tempdir().expect("Unable to create temp directory");

    let yaml_paths: Vec<PathBuf> = yamls.iter().map(PathBuf::from).collect();
    let yaml_refs: Vec<&Path> = yaml_paths.iter().map(|p| p.as_path()).collect();

    yaml_lv_style(&yaml_refs, tmp_dir.path());

    let expected_dir = PathBuf::from(expected_dir_path);
    compare_directory(&expected_dir, tmp_dir.path());
}