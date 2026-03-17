include(FetchContent)

function(get_yaml_lv_style version)
    message(STATUS "Searching for yaml_lv_style (version: ${version})...")

    if("${version}" STREQUAL "latest")
        set(api_url "https://api.github.com/repos/tanguy-rdt/yaml_lv_style/releases/latest")
    else()
        set(api_url "https://api.github.com/repos/tanguy-rdt/yaml_lv_style/releases/tags/${version}")
    endif()

    set(json_file "${CMAKE_BINARY_DIR}/yaml_lv_style_release.json")

    file(DOWNLOAD "${api_url}" "${json_file}"
        STATUS download_status)

    list(GET download_status 0 status_code)
    if(NOT status_code EQUAL 0)
        message(FATAL_ERROR "Error retrieving GitHub information: ${download_status}")
    endif()

    file(READ "${json_file}" release_json)
    string(JSON version GET "${release_json}" "tag_name")

    if(APPLE)
        set(os "macos")
    elseif(UNIX)
        set(os "linux")
    else()
        message(FATAL_ERROR "Operating system not supported")
    endif()

    set(arch ${CMAKE_HOST_SYSTEM_PROCESSOR})
    if(arch MATCHES "x86_64|amd64|AMD64")
        set(arch "x86_64")
    elseif(arch MATCHES "arm64|aarch64")
        set(arch "arm64")
    else()
        message(FATAL_ERROR "Architecture not supported: ${arch}")
    endif()

    set(archive_dir "${CMAKE_BINARY_DIR}/external/yaml_lv_style/archives")
    set(extract_dir "${CMAKE_BINARY_DIR}/external/yaml_lv_style/bin/${version}")
    set(archive_file "${archive_dir}/yaml_lv_style_${version}-${os}-${arch}.tar.gz")

    set(YAML_LV_STYLE_EXECUTABLE "${extract_dir}/yaml_lv_style/yaml_lv_style" CACHE INTERNAL "")

    if (EXISTS "${YAML_LV_STYLE_EXECUTABLE}")
        message(STATUS "yaml_lv_style (version: ${version}) found in: ${extract_dir}")
    else ()
        string(JSON asset_count LENGTH "${release_json}" "assets")
        math(EXPR asset_count "${asset_count} - 1")

        set(search_pattern ".*-${os}-${arch}\\.tar\\.gz$")
        
        set(found_url "")
        foreach(index RANGE ${asset_count})
            string(JSON asset_name GET "${release_json}" "assets" ${index} "name")
            if(asset_name MATCHES "${search_pattern}")
                string(JSON found_url GET "${release_json}" "assets" ${index} "url")
                break()
            endif()
        endforeach()

        if(NOT found_url)
            if (version EQUAL "latest")
                message(FATAL_ERROR "Unable to find archive ${search_pattern} for the latest version")
            else ()
                message(FATAL_ERROR "Unable to find archive ${search_pattern} for version ${version}")
            endif ()
        endif()

        file(MAKE_DIRECTORY "${archive_dir}")

        message(STATUS "Downloading yaml_lv_style from ${found_url}...")
        file(DOWNLOAD "${found_url}" "${archive_file}"
            HTTPHEADER "Accept: application/octet-stream"
            TLS_VERIFY ON
            STATUS download_status
        )

        list(GET download_status 0 status_code)
        if(NOT status_code EQUAL 0)
            message(FATAL_ERROR "Download error: ${download_status}")
        endif()

        message(STATUS "Extracting to ${extract_dir}...")
        file(MAKE_DIRECTORY "${extract_dir}")
        file(ARCHIVE_EXTRACT
            INPUT "${archive_file}"
            DESTINATION "${extract_dir}"
        )

        execute_process(COMMAND chmod +x "${YAML_LV_STYLE_EXECUTABLE}")
    endif ()


    list(APPEND CMAKE_MODULE_PATH "${extract_dir}/yaml_lv_style/cmake")
    find_package(YamlLvStyle REQUIRED)
endfunction()
