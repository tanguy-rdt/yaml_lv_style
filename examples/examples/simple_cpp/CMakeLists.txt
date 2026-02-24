add_executable(simple_cpp_example)

yaml_lv_style_generate_cpp(simple_cpp_style
    FORMAT
    FILES
        "${CMAKE_CURRENT_SOURCE_DIR}/styles/dark.yaml"
        "${CMAKE_CURRENT_SOURCE_DIR}/styles/light.yaml"
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