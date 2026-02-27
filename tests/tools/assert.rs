use std::path::Path;
use std::path::PathBuf;

use tempfile::tempdir;

use super::helper::compare_directory;
use super::helper::yaml_lv_style;

#[macro_export]
macro_rules! assert_stylesheet {
    ($yaml:expr) => {
        let test_name = std::thread::current()
            .name()
            .unwrap_or("unknown_test")
            .to_string();
        let full_function_name = test_name.split("::").last().unwrap_or(&test_name);
        let short_test_name = full_function_name
            .strip_prefix("test_")
            .unwrap_or(full_function_name);

        $crate::tools::assert::validate_yaml_output($yaml, &short_test_name);
    };
}

#[macro_export]
macro_rules! assert_stylesheets {
    ($yamls:expr) => {
        let test_name = std::thread::current()
            .name()
            .unwrap_or("unknown_test")
            .to_string();
        let full_function_name = test_name.split("::").last().unwrap_or(&test_name);
        let short_test_name = full_function_name
            .strip_prefix("test_")
            .unwrap_or(full_function_name);

        $crate::tools::assert::validate_yamls_output($yamls, &short_test_name);
    };
}

pub fn validate_yaml_output(yaml: &str, short_test_name: &str) {
    compare_generated_to_expected(&[yaml], "c", short_test_name);
    compare_generated_to_expected(&[yaml], "cpp", short_test_name);
}

pub fn validate_yamls_output(yamls: &[&str], short_test_name: &str) {
    compare_generated_to_expected(yamls, "c", short_test_name);
    compare_generated_to_expected(yamls, "cpp", short_test_name);
}

pub fn compare_generated_to_expected(yamls: &[&str], lang: &str, short_test_name: &str) {
    let tmp_dir = tempdir().expect("Unable to create temp directory");
    let tmp_dir_path = tmp_dir.path().join(lang).join(short_test_name);
    let expected_dir = PathBuf::from(format!(
        "tests/expected_result/{}/{}",
        lang, short_test_name
    ));

    let yaml_paths: Vec<PathBuf> = yamls.iter().map(PathBuf::from).collect();
    let yaml_refs: Vec<&Path> = yaml_paths.iter().map(|p| p.as_path()).collect();

    yaml_lv_style(lang, &yaml_refs, &tmp_dir_path);

    compare_directory(&expected_dir, &tmp_dir_path);
}
