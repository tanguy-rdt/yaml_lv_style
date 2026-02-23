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

    cmake_parse_arguments(yaml "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})

    if(NOT yaml_FILES)
        message(FATAL_ERROR "yaml_lv_style_generate: You must specify at least one file in FILES")
    endif()

    if(NOT yaml_OUTPUT_DIR)
        set(yaml_OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated/generated_styles")
    endif()

    set(args -p -i ${yaml_FILES} -o ${yaml_OUTPUT_DIR} -l ${lang})

    if(yaml_NAMESPACE AND "${lang}" STREQUAL "cpp")
        list(APPEND args -n ${yaml_NAMESPACE})
    endif()

    if("FORMAT" IN_LIST yaml_KEYWORDS_MISSING_VALUES OR yaml_FORMAT)
        list(APPEND args -f)

        if(yaml_FORMAT)
            list(APPEND args ${yaml_FORMAT})
        endif()
    endif()

    _yaml_lv_style_run("${yaml_OUTPUT_DIR}" "${args}" generated_sources)
    _yaml_lv_style_make_lib("${target_name}" "${yaml_ALIAS}" "${yaml_OUTPUT_DIR}" "${generated_sources}")
endfunction()

function(_yaml_lv_style_run output_dir args generated_sources)
    file(MAKE_DIRECTORY ${output_dir})

    execute_process(
        COMMAND ${YAML_LV_STYLE_EXECUTABLE} ${args}
        WORKING_DIRECTORY ${CMAKE_SOURCE_DIR}
        RESULT_VARIABLE yaml_lv_style_result
        OUTPUT_VARIABLE generated_files_list
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )

    if(NOT yaml_lv_style_result EQUAL 0)
        message(FATAL_ERROR "yaml_lv_style_generate: Error generating styles: ${yaml_lv_style_result}")
    endif()

    string(REPLACE "\n" ";" generated_files_list "${generated_files_list}")

    set(sources "")
    foreach(file ${generated_files_list})
        if(file MATCHES ".*\\.cpp$" OR file MATCHES ".*\\.c$")
            list(APPEND sources "${file}")
        endif()
    endforeach()

    set(${generated_sources} "${sources}" PARENT_SCOPE)
endfunction()

function(_yaml_lv_style_make_lib target_name alias output_dir sources)
    if(NOT TARGET lvgl)
        message(FATAL_ERROR "yaml_lv_style_generate: The target 'lvgl' is required for the style generator
        cannot be found.")
    endif()

    add_library(${target_name})

    if (alias)
        add_library(${alias} ALIAS ${target_name})
    endif ()

    target_sources(${target_name}
        PRIVATE
        ${sources}
    )

    target_include_directories(${target_name}
        PUBLIC
        ${output_dir}/styles/include/
        ${output_dir}/stylesheets/include/
    )

    target_link_libraries(${target_name}
        PUBLIC
        lvgl
    )
endfunction()