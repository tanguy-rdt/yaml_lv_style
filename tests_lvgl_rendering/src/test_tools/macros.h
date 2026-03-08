#ifndef TEST_MACROS_H
#define TEST_MACROS_H

#include <catch2/catch_test_macros.hpp>
#include <lvgl.h>
#include <filesystem>

#define REQUIRE_SCREENSHOT_COMPARE(...)                                          \
    do {                                                                         \
        std::string _name = []() -> std::string {                                \
            if constexpr (std::string_view(#__VA_ARGS__).empty())                \
                return "default";                                                \
            else                                                                 \
                return std::string(__VA_ARGS__);                                 \
        }();                                                                     \
        std::string dir  = std::filesystem::path(__FILE__).stem().string();      \
        std::string test = Catch::getResultCapture().getCurrentTestName();       \
        std::string path = dir + "/" + test + "_" + _name                        \
            + "_" + GEN_LANG                                                     \
            + "_" + GEN_STYLE + ".png";                                          \
        REQUIRE(lv_test_screenshot_compare(path.c_str()));                       \
    } while(false)

#endif // TEST_MACROS_H
