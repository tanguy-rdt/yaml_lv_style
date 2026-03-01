cmake_minimum_required(VERSION 3.31)

project(yaml_lv_style_example
    DESCRIPTION  "LVGL style generator from YAML examples"
    LANGUAGES    C CXX
    HOMEPAGE_URL https://github.com/tanguy-rdt/yaml_lv_style
)

set(CMAKE_C_STANDARD 17)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_C_EXTENSIONS OFF)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

configure_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/conf/lv_conf.h.in"
    "${CMAKE_BINARY_DIR}/generated/lvgl/lv_conf.h"
)

include(FetchContent)

FetchContent_Declare(
    SDL2
    GIT_REPOSITORY https://github.com/libsdl-org/SDL.git
    GIT_TAG        release-2.32.4
)
set(SDL_AUDIO OFF CACHE BOOL "" FORCE)
set(SDL_JOYSTICK OFF CACHE BOOL "" FORCE)
set(SDL_HAPTIC OFF CACHE BOOL "" FORCE)
set(SDL_SENSOR OFF CACHE BOOL "" FORCE)
set(SDL_TEST OFF CACHE BOOL "" FORCE)

FetchContent_Declare(
    lvgl
    GIT_REPOSITORY https://github.com/lvgl/lvgl.git
    GIT_TAG        v9.4.0
)
set(CONFIG_LV_BUILD_DEMOS OFF CACHE BOOL "" FORCE)
set(CONFIG_LV_BUILD_EXAMPLES OFF CACHE BOOL "" FORCE)
set(LV_BUILD_CONF_PATH "${CMAKE_BINARY_DIR}/generated/lvgl/lv_conf.h" CACHE STRING "" FORCE)

FetchContent_MakeAvailable(SDL2 lvgl)

target_link_libraries(lvgl PRIVATE SDL2)

include(cmake/get-yaml_lv_style.cmake)
get_yaml_lv_style(latest)

add_subdirectory(examples/grid)
add_subdirectory(examples/simple_c)
add_subdirectory(examples/simple_cpp)