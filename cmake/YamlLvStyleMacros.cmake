function(yaml_lv_style_generate target_name)
    if(NOT TARGET lvgl)
        message(FATAL_ERROR "yaml_lv_style_generate: The target 'lvgl' required for the style generator
        cannot be found.")
    endif()

    set(options)
    set(oneValueArgs OUTPUT_DIR NAMESPACE FORMAT)
    set(multiValueArgs FILES)

    cmake_parse_arguments(yaml "${options}" "${oneValueArgs}" "${multiValueArgs}" ${ARGN})

    if(NOT yaml_FILES)
        message(FATAL_ERROR "yaml_lv_style_generate: You must specify at least one file in FILES")
    endif()

    if(NOT yaml_OUTPUT_DIR)
        set(yaml_OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated/generated_styles")
    endif()

    set(cmd ${YAML_LV_STYLE_EXECUTABLE} -p -i ${yaml_FILES} -o ${yaml_OUTPUT_DIR})

    if(yaml_NAMESPACE)
        list(APPEND cmd -n ${yaml_NAMESPACE})
    endif()

    if("FORMAT" IN_LIST yaml_KEYWORDS_MISSING_VALUES OR yaml_FORMAT)
        list(APPEND cmd -f)

        if(yaml_FORMAT)
            list(APPEND cmd ${yaml_FORMAT})
        endif()
    endif()

    file(MAKE_DIRECTORY ${yaml_OUTPUT_DIR})

    execute_process(
        COMMAND ${cmd}
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

    add_library(${target_name})

    target_sources(${target_name}
        PRIVATE
        ${sources}
    )

    target_include_directories(${target_name}
        PUBLIC
        ${yaml_OUTPUT_DIR}/styles/include/
        ${yaml_OUTPUT_DIR}/stylesheets/include/
    )

    target_link_libraries(${target_name}
        PUBLIC
        lvgl
    )
endfunction()