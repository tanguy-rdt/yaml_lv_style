function(yaml_lv_style_add_test_case test_case)
    foreach(lang IN ITEMS c cpp)
        foreach(use_const_style IN ITEMS true false)
            if(use_const_style)
                set(variant "${test_case}_${lang}_const_style")
            else()
                set(variant "${test_case}_${lang}_dyn_style")
            endif()

            _yaml_lv_style_configure_yamls("${test_case}" "${variant}" "${use_const_style}" configured_yamls)
            _yaml_lv_style_run_codegen("${test_case}" "${variant}" "${lang}" ${configured_yamls})
            _yaml_lv_style_register_target("${test_case}" "${variant}" "${lang}" "${use_const_style}" ${configured_yamls})
        endforeach()
    endforeach()
endfunction()

function(_yaml_lv_style_configure_yamls test_case variant use_const_style out_configured_yamls)
    file(GLOB raw_yamls CONFIGURE_DEPENDS
        "src/test_cases/${test_case}/yaml/*.yaml"
    )

    if(NOT raw_yamls)
        message(WARNING "[yaml_lv_style_test] No YAML files found for test '${test_case}'")
        set(${out_configured_yamls} "" PARENT_SCOPE)
        return()
    endif()

    set(configured_yamls "")
    foreach(raw_yaml IN LISTS raw_yamls)
        file(READ "${raw_yaml}" raw_yaml_content)
        if(NOT raw_yaml_content MATCHES "@CONST_STYLE@")
            message(WARNING "[yaml_lv_style_test] '${raw_yaml}' is missing 'const: @CONST_STYLE@' — skipping variant '${variant}'")
            set(${out_configured_yamls} "" PARENT_SCOPE)
            return()
        endif()

        get_filename_component(yaml_filename "${raw_yaml}" NAME)
        set(configured_yaml "${CMAKE_BINARY_DIR}/generated/yaml/${test_case}/${variant}/${yaml_filename}")
        set(CONST_STYLE "${use_const_style}")
        configure_file("${raw_yaml}" "${configured_yaml}" @ONLY)
        list(APPEND configured_yamls "${configured_yaml}")
    endforeach()

    set(${out_configured_yamls} "${configured_yamls}" PARENT_SCOPE)
endfunction()

function(_yaml_lv_style_run_codegen test_case variant lang)
    set(configured_yamls ${ARGN})
    set(codegen_lib   "${variant}_codegen_lib")
    set(codegen_outdir "${CMAKE_BINARY_DIR}/generated/styles/${test_case}/${variant}/styles_gen")

    if(lang STREQUAL "cpp")
        yaml_lv_style_generate_cpp("${codegen_lib}" FORMAT FILES ${configured_yamls} OUTPUT_DIR "${codegen_outdir}")
    else()
        yaml_lv_style_generate_c("${codegen_lib}"   FORMAT FILES ${configured_yamls} OUTPUT_DIR "${codegen_outdir}")
    endif()

    add_dependencies("${codegen_lib}" yaml_lv_style_rust_build)
endfunction()

function(_yaml_lv_style_register_target test_case variant lang use_const_style)
    set(codegen_lib "${variant}_codegen_lib")

    add_executable("${variant}"
        src/main.cpp
        src/test_assets/test_img_lvgl_logo_png.c
        "src/test_cases/${test_case}/${test_case}.cpp"
    )

    target_compile_definitions("${variant}"
        PRIVATE
            GEN_LANG="${lang}"
            GEN_STYLE="$<IF:$<STREQUAL:${use_const_style},true>,const_style,dyn_style>"
    )

    target_include_directories("${variant}" PRIVATE src/test_tools/)

    target_link_libraries("${variant}"
        PRIVATE
        Catch2::Catch2WithMain
        lvgl
        "${codegen_lib}"
    )

    catch_discover_tests("${variant}" TEST_PREFIX "${variant}_")

    message(STATUS "[yaml_lv_style_test] Registered: ${variant}")
endfunction()