add_executable(simple_cpp_example)

yaml_lv_style_generate_cpp(simple_cpp_style
    CONFIG styles/config.yaml
    FILES
        styles/dark.yaml
        styles/light.yaml
    OUTPUT_DIR
        src/styles
)

target_sources(simple_cpp_example
    PRIVATE
        src/main.cpp
)

target_link_libraries(simple_cpp_example
    PRIVATE
        lvgl
        simple_cpp_style
)