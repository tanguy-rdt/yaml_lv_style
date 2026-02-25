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

#[test]
fn test_all_states() {
    assert_stylesheet!("tests/yaml_stylesheets/all_states.yaml");
}

#[test]
fn test_all_properties() {
    assert_stylesheets!(&[
        "tests/yaml_stylesheets/properties_arc.yaml",
        "tests/yaml_stylesheets/properties_background.yaml",
        "tests/yaml_stylesheets/properties_border.yaml",
        "tests/yaml_stylesheets/properties_flex.yaml",
        "tests/yaml_stylesheets/properties_grid.yaml",
        "tests/yaml_stylesheets/properties_image.yaml",
        "tests/yaml_stylesheets/properties_line.yaml",
        "tests/yaml_stylesheets/properties_margin.yaml",
        "tests/yaml_stylesheets/properties_miscellaneous.yaml",
        "tests/yaml_stylesheets/properties_outline.yaml",
        "tests/yaml_stylesheets/properties_padding.yaml",
        "tests/yaml_stylesheets/properties_size_and_position.yaml",
        "tests/yaml_stylesheets/properties_text.yaml",
    ]);
}
