cmake_minimum_required(VERSION 3.31)

project(yaml_lv_style_example
    DESCRIPTION  "Example of using yaml_lv_style"
    LANGUAGES    C CXX
    HOMEPAGE_URL https://github.com/tanguy-rdt/yaml_lv_style
)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/conf/lv_conf.h.in"
    "${CMAKE_BINARY_DIR}/generated/lvgl/lv_conf.h"
)

include(cmake/CPM.cmake)

CPMAddPackage(
    NAME SDL2
    GITHUB_REPOSITORY libsdl-org/SDL
    GIT_TAG release-2.32.4
    OPTIONS
    "SDL_SHARED ON"
)

CPMAddPackage(
    NAME lvgl
    GITHUB_REPOSITORY lvgl/lvgl
    GIT_TAG v9.4.0
    OPTIONS
        "CONFIG_LV_BUILD_DEMOS OFF"
        "CONFIG_LV_BUILD_EXAMPLES OFF"
        "LV_BUILD_CONF_PATH ${CMAKE_BINARY_DIR}/generated/lvgl/lv_conf.h"
)

target_link_libraries(lvgl PRIVATE SDL2)

include(cmake/get-yaml_lv_style.cmake)
get_yaml_lv_style(latest)

yaml_lv_style_generate(styles
    FORMAT
    FILES
        "${CMAKE_CURRENT_SOURCE_DIR}/styles/dark.yaml"
        "${CMAKE_CURRENT_SOURCE_DIR}/styles/light.yaml"
)

add_executable(${PROJECT_NAME})

target_sources(${PROJECT_NAME}
    PRIVATE
        src/main.cpp
)

target_link_libraries(${PROJECT_NAME}
    PRIVATE
        lvgl
        styles
)