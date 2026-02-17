find_program(YAML_LV_STYLE_EXECUTABLE NAMES yaml_lv_style)

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(YamlLvStyle
    DEFAULT_MSG
        YAML_LV_STYLE_EXECUTABLE
)

if(YamlLvStyle_FOUND)
    include("${CMAKE_CURRENT_LIST_DIR}/YamlLvStyleMacros.cmake")
endif()