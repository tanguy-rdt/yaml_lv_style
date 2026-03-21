add_executable(simple_c_example)

yaml_lv_style_generate_c(simple_c_styles
    FORMAT
    FILES
        styles/dark.yaml
        styles/light.yaml
    OUTPUT_DIR
        src/styles
)

target_sources(simple_c_example
    PRIVATE
        src/main.c
)

target_link_libraries(simple_c_example
    PRIVATE
        lvgl
        simple_c_styles
)