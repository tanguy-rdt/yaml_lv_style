cmake_minimum_required(VERSION 3.31)

project(yaml_lv_style_tests
    LANGUAGES    C CXX
    HOMEPAGE_URL https://github.com/tanguy-rdt/yaml_lv_style
)

set(CMAKE_C_STANDARD 17)
set(CMAKE_CXX_STANDARD 23)

option(TEST_C_LANG "Generation of C style sheets and execution of associated tests" OFF)
option(TEST_CPP_LANG "Generation of C++ style sheets and execution of associated tests" OFF)
option(TEST_CONST_STYLE "Generation of constant style sheets and execution of associated tests" OFF)
option(TEST_DYN_STYLE "Generation of dynamic style sheets and execution of associated tests" OFF)

set(REPOSITORY ${CMAKE_SOURCE_DIR}/..)
set(RUST_BUILD_DIR ${CMAKE_BINARY_DIR}/rust)
set(YAML_LV_STYLE_EXECUTABLE ${RUST_BUILD_DIR}/release/yaml_lv_style)

add_custom_target(yaml_lv_style_rust_build
    COMMAND cargo build --release --target-dir ${RUST_BUILD_DIR} --features macros
    WORKING_DIRECTORY ${REPOSITORY}
    BYPRODUCTS ${YAML_LV_STYLE_EXECUTABLE}
    COMMENT "Building yaml_lv_style (Rust)"
    VERBATIM
)

include(${REPOSITORY}/cmake/YamlLvStyleMacros.cmake)

include(cmake/dependencies.cmake)

include(CTest)
include(Catch)

include(cmake/yaml_lv_style_add_test_case.cmake)

yaml_lv_style_add_test_case(test_size_and_position)
yaml_lv_style_add_test_case(test_padding)
yaml_lv_style_add_test_case(test_margin)
yaml_lv_style_add_test_case(test_background)
yaml_lv_style_add_test_case(test_border)
yaml_lv_style_add_test_case(test_outline)
yaml_lv_style_add_test_case(test_shadow)
yaml_lv_style_add_test_case(test_image)
yaml_lv_style_add_test_case(test_line)
yaml_lv_style_add_test_case(test_arc)
yaml_lv_style_add_test_case(test_text)
yaml_lv_style_add_test_case(test_miscellaneous)
yaml_lv_style_add_test_case(test_flex)
yaml_lv_style_add_test_case(test_grid)
