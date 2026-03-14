function(yaml_lv_style_generate_cpp target_name)
    _yaml_lv_style_generate(${target_name} "cpp" ${ARGN})
endfunction()

function(yaml_lv_style_generate_c target_name)
    _yaml_lv_style_generate(${target_name} "c" ${ARGN})
endfunction()

function(_yaml_lv_style_generate target_name lang)
    set(options)
    set(oneValueArgs ALIAS OUTPUT_DIR NAMESPACE FORMAT)
    set(multiValueArgs FILES)

    cmake_parse_arguments(args "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})

    if(NOT args_FILES)
        message(FATAL_ERROR "yaml_lv_style_generate: You must specify at least one file in FILES")
    endif()

    if(NOT args_OUTPUT_DIR)
        set(args_OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated/generated_styles")
    endif()

    set(args -i ${args_FILES} -o ${args_OUTPUT_DIR} -l ${lang})

    if(args_NAMESPACE AND "${lang}" STREQUAL "cpp")
        list(APPEND args -n ${args_NAMESPACE})
    endif()

    if("FORMAT" IN_LIST yaml_KEYWORDS_MISSING_VALUES OR args_FORMAT)
        list(APPEND args -f)

        if(args_FORMAT)
            list(APPEND args ${args_FORMAT})
        endif()
    endif()

    _yaml_lv_style_expected_generated_files("${args_OUTPUT_DIR}" "${lang}" "${args_FILES}" generated_files generated_sources)
    _yaml_lv_style_run("${args_FILES}" "${args_OUTPUT_DIR}" "${args}" "${generated_files}")
    _yaml_lv_style_make_lib("${target_name}" "${args_ALIAS}" "${args_OUTPUT_DIR}" "${generated_sources}")
endfunction()

function(_yaml_lv_style_expected_generated_files output_dir lang yaml_files generated_files generated_sources)
    set(gen_files "")

    foreach(yaml_file IN LISTS yaml_files)
        get_filename_component(yaml_name "${yaml_file}" NAME_WE)
        get_filename_component(parent_output_dir_name "${output_dir}" NAME)

        list(APPEND gen_files
            "${output_dir}/styles/include/${parent_output_dir_name}/styles.h"
        )

        list(APPEND gen_files
            "${output_dir}/stylesheets/include/${parent_output_dir_name}/stylesheet_${yaml_name}.h"
            "${output_dir}/stylesheets/include/${parent_output_dir_name}/stylesheets.h"
        )

        if("${lang}" STREQUAL "c")
            list(APPEND gen_files
                "${output_dir}/stylesheets/src/stylesheet_${yaml_name}.c"
                "${output_dir}/stylesheets/src/stylesheets.c"
            )
        else()
            list(APPEND gen_files
                "${output_dir}/stylesheets/src/stylesheet_${yaml_name}.cpp"
                "${output_dir}/stylesheets/src/stylesheets.cpp"
            )
        endif()
    endforeach()

    list(REMOVE_DUPLICATES gen_files)

    set(gen_sources "")
    foreach(file ${gen_files})
        if(file MATCHES ".*\\.cpp$" OR file MATCHES ".*\\.c$")
            list(APPEND gen_sources "${file}")
        endif()
    endforeach()

    set(${generated_files} "${gen_files}" PARENT_SCOPE)
    set(${generated_sources} "${gen_sources}" PARENT_SCOPE)
endfunction()

function(_yaml_lv_style_run yaml_files output_dir args generated_files)
    file(MAKE_DIRECTORY ${output_dir})

    add_custom_command(
        OUTPUT ${generated_files}
        COMMAND ${YAML_LV_STYLE_EXECUTABLE} ${args}
        DEPENDS ${yaml_files}
        COMMENT "Generating LVGL styles from YAML"
        VERBATIM
    )
endfunction()

function(_yaml_lv_style_make_lib target_name alias output_dir generated_sources)
    if(NOT TARGET lvgl)
        message(FATAL_ERROR "yaml_lv_style_generate: The target 'lvgl' is required for the style generator
        cannot be found.")
    endif()

    set(names_target "${target_name}_names")
    add_library(${names_target} INTERFACE)
    target_include_directories(${names_target}
        INTERFACE
        ${output_dir}/styles/include/
    )

    add_library(${target_name})
    target_sources(${target_name} PRIVATE ${generated_sources})
    target_include_directories(${target_name}
        PUBLIC
        ${output_dir}/stylesheets/include/
    )
    target_link_libraries(${target_name} PUBLIC lvgl ${names_target})

    if(alias)
        add_library(${alias} ALIAS ${target_name})
        add_library(${alias}_names ALIAS ${names_target})
    else()
        add_library(${target_name}::${target_name} ALIAS ${target_name})
        add_library(${target_name}::style_names ALIAS ${names_target})
    endif()
endfunction()