mod tools;

#[test]
fn test_multi_selectors_generation() {
    assert_stylesheet!("tests/yaml/test_selectors.yaml");
}

#[test]
fn test_const_and_dyn_generation() {
    assert_stylesheet!("tests/yaml/test_const_dyn.yaml");
}

#[test]
fn test_colors_values_generation() {
    assert_stylesheet!("tests/yaml/test_colors.yaml");
}

#[test]
fn test_coords_values_generation() {
    assert_stylesheet!("tests/yaml/test_coords.yaml");
}

#[test]
fn test_enum_values_generation() {
    assert_stylesheet!("tests/yaml/test_enums.yaml");
}

#[test]
fn test_grid_values_generation() {
    assert_stylesheet!("tests/yaml/test_grid.yaml");
}

#[test]
fn test_opa_values_generation() {
    assert_stylesheet!("tests/yaml/test_opa.yaml");
}

#[test]
fn test_multi_stylesheet_generation() {
    assert_stylesheets!(&[
        "tests/yaml/test_selectors.yaml",
        "tests/yaml/test_const_dyn.yaml"
    ]);
}
