#ifndef TEST_MACROS_H
#define TEST_MACROS_H

#include <catch2/catch_test_macros.hpp>
#include <lvgl.h>
#include <filesystem>

#include "styles_gen/stylesheets_macros.h"

struct InitStyleSheetFixture {
    InitStyleSheetFixture() { INIT_STYLE_SHEETS(); }
};

#define TEST(name)                              \
    TEST_CASE_METHOD(InitStyleSheetFixture, name)

#define TEST_CHECK(...)       \
    do {                            \
        lv_refr_now(nullptr);       \
        CHECK(__VA_ARGS__);       \
    } while(false)

#define TEST_CHECK_SCREENSHOT_COMPARE(...)                                 \
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
        lv_refr_now(nullptr);                                                    \
        CHECK(lv_test_screenshot_compare(path.c_str()));                       \
    } while(false)

#endif // TEST_MACROS_H
