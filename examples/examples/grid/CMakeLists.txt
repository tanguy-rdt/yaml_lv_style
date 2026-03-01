add_executable(grid_example)

yaml_lv_style_generate_c(grid_styles
    FORMAT
    FILES
        "${CMAKE_CURRENT_SOURCE_DIR}/styles/grid.yaml"
    OUTPUT_DIR
        "${CMAKE_CURRENT_SOURCE_DIR}/src/generated_styles"
)

target_sources(grid_example
    PRIVATE
        src/main.c
)

target_link_libraries(grid_example
    PRIVATE
        lvgl
        grid_styles
)