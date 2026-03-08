#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_line_basic") {
    INIT_STYLE_SHEETS();

    static const lv_point_precise_t points[] = {
        {10, 50}, {100, 10}, {200, 50}, {290, 10}
    };

    lv_obj_t* line = lv_line_create(lv_screen_active());
    lv_line_set_points(line, points, 4);
    lv_obj_center(line);
    SET_LINE_STYLE(line, ENUM_LINE_STYLE_TEST_LINE_BASIC);

    lv_obj_add_state(line, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_line_width(line, LV_PART_MAIN)   == 2);
    REQUIRE(lv_obj_get_style_line_opa(line, LV_PART_MAIN)     == LV_OPA_COVER);
    REQUIRE(lv_obj_get_style_line_rounded(line, LV_PART_MAIN) == false);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(line, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_line_width(line, LV_PART_MAIN)   == 5);
    REQUIRE(lv_obj_get_style_line_opa(line, LV_PART_MAIN)     == LV_OPA_50);
    REQUIRE(lv_obj_get_style_line_rounded(line, LV_PART_MAIN) == true);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(line, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_line_width(line, LV_PART_MAIN) == 1);
    REQUIRE(lv_obj_get_style_line_opa(line, LV_PART_MAIN)   == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_line_dash") {
    INIT_STYLE_SHEETS();

    static const lv_point_precise_t points[] = {
        {10, 50}, {290, 50}
    };

    lv_obj_t* line = lv_line_create(lv_screen_active());
    lv_line_set_points(line, points, 2);
    lv_obj_center(line);
    SET_LINE_STYLE(line, ENUM_LINE_STYLE_TEST_LINE_DASH);

    lv_obj_add_state(line, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_line_dash_width(line, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_line_dash_gap(line, LV_PART_MAIN)   == 0);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(line, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_line_dash_width(line, LV_PART_MAIN) == 10);
    REQUIRE(lv_obj_get_style_line_dash_gap(line, LV_PART_MAIN)   == 5);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(line, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_line_dash_width(line, LV_PART_MAIN) == 5);
    REQUIRE(lv_obj_get_style_line_dash_gap(line, LV_PART_MAIN)   == 15);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}