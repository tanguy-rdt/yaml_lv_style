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

    set(gen_files_list "${CMAKE_CURRENT_BINARY_DIR}/generated/yls/gen_list.txt")

    _yaml_lv_style_run("${args_FILES}" "${args_OUTPUT_DIR}" "${args}" "${gen_files_list}")
    _yaml_lv_style_make_lib("${target_name}" "${args_ALIAS}" "${args_OUTPUT_DIR}" "${gen_files_list}")
endfunction()

function(_yaml_lv_style_run yaml_files output_dir args gen_files_list)
    file(MAKE_DIRECTORY ${output_dir})

    set_property(DIRECTORY APPEND PROPERTY CMAKE_CONFIGURE_DEPENDS ${yaml_files})

    execute_process(
        COMMAND ${YAML_LV_STYLE_EXECUTABLE} ${args} --output-list ${gen_files_list}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        RESULT_VARIABLE yaml_lv_style_result
    )

    if(NOT yaml_lv_style_result EQUAL 0)
        message(FATAL_ERROR "yaml_lv_style_generate: Error generating styles: ${yaml_lv_style_result}")
    endif()
endfunction()

function(_yaml_lv_style_make_lib target_name alias output_dir gen_files_list)
    if(NOT TARGET lvgl)
        message(FATAL_ERROR "yaml_lv_style_generate: The target 'lvgl' is required for the style generator
        cannot be found.")
    endif()

    file(STRINGS ${gen_files_list} generated_files)

    set(generated_sources "")
    set(generated_headers "")
    foreach(file ${generated_files})
        if(file MATCHES "\\.cpp$" OR file MATCHES "\\.c$")
            list(APPEND generated_sources ${file})
        endif()
    endforeach()

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