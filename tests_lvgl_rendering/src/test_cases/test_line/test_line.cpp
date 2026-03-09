#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST("test_line_basic") {
    static const lv_point_precise_t points[] = {
        {10, 50}, {100, 10}, {200, 50}, {290, 10}
    };

    lv_obj_t* line = lv_line_create(lv_screen_active());
    lv_line_set_points(line, points, 4);
    lv_obj_center(line);
    SET_LINE_STYLE(line, ENUM_LINE_STYLE_TEST_LINE_BASIC);

    lv_obj_add_state(line, LV_STATE_DEFAULT);
    TEST_CHECK(lv_obj_get_style_line_width(line, LV_PART_MAIN)   == 2);
    TEST_CHECK(lv_obj_get_style_line_opa(line, LV_PART_MAIN)     == LV_OPA_COVER);
    TEST_CHECK(lv_obj_get_style_line_rounded(line, LV_PART_MAIN) == false);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(line, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_line_width(line, LV_PART_MAIN)   == 5);
    TEST_CHECK(lv_obj_get_style_line_opa(line, LV_PART_MAIN)     == LV_OPA_50);
    TEST_CHECK(lv_obj_get_style_line_rounded(line, LV_PART_MAIN) == true);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(line, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_line_width(line, LV_PART_MAIN) == 1);
    TEST_CHECK(lv_obj_get_style_line_opa(line, LV_PART_MAIN)   == LV_OPA_TRANSP);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_line_dash") {
    static const lv_point_precise_t points[] = {
        {10, 50}, {290, 50}
    };

    lv_obj_t* line = lv_line_create(lv_screen_active());
    lv_line_set_points(line, points, 2);
    lv_obj_center(line);
    SET_LINE_STYLE(line, ENUM_LINE_STYLE_TEST_LINE_DASH);

    lv_obj_add_state(line, LV_STATE_DEFAULT);
    TEST_CHECK(lv_obj_get_style_line_dash_width(line, LV_PART_MAIN) == 0);
    TEST_CHECK(lv_obj_get_style_line_dash_gap(line, LV_PART_MAIN)   == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(line, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_line_dash_width(line, LV_PART_MAIN) == 10);
    TEST_CHECK(lv_obj_get_style_line_dash_gap(line, LV_PART_MAIN)   == 5);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(line, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_line_dash_width(line, LV_PART_MAIN) == 5);
    TEST_CHECK(lv_obj_get_style_line_dash_gap(line, LV_PART_MAIN)   == 15);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}