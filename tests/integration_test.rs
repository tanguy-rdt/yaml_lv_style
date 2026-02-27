mod tools;

#[test]
fn test_dyn_style_generation() {
    assert_stylesheet!("tests/yaml_stylesheets/dyn.yaml");
}

#[test]
fn test_const_style_generation() {
    assert_stylesheet!("tests/yaml_stylesheets/const.yaml");
}

#[test]
fn test_const_and_dyn_style_generation() {
    assert_stylesheet!("tests/yaml_stylesheets/const_and_dyn.yaml");
}

#[test]
fn test_multi_stylesheet_generation() {
    assert_stylesheets!(&[
        "tests/yaml_stylesheets/const.yaml",
        "tests/yaml_stylesheets/dyn.yaml"
    ]);
}
