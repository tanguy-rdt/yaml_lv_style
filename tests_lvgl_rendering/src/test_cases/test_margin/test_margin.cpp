#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST("test_margin") {
    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 400, 400);
    lv_obj_center(container);
    lv_obj_set_flex_flow(container, LV_FLEX_FLOW_ROW_WRAP);

    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_set_size(obj, 100, 100);
    SET_MARGIN_STYLE(obj, ENUM_MARGIN_STYLE_TEST_MARGIN);

    TEST_CHECK(lv_obj_get_style_margin_top(obj, LV_PART_MAIN)    == 10);
    TEST_CHECK(lv_obj_get_style_margin_bottom(obj, LV_PART_MAIN) == 10);
    TEST_CHECK(lv_obj_get_style_margin_left(obj, LV_PART_MAIN)   == 10);
    TEST_CHECK(lv_obj_get_style_margin_right(obj, LV_PART_MAIN)  == 10);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_margin_top(obj, LV_PART_MAIN)    == 30);
    TEST_CHECK(lv_obj_get_style_margin_bottom(obj, LV_PART_MAIN) == 30);
    TEST_CHECK(lv_obj_get_style_margin_left(obj, LV_PART_MAIN)   == 30);
    TEST_CHECK(lv_obj_get_style_margin_right(obj, LV_PART_MAIN)  == 30);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_margin_top(obj, LV_PART_MAIN)    == 0);
    TEST_CHECK(lv_obj_get_style_margin_bottom(obj, LV_PART_MAIN) == 0);
    TEST_CHECK(lv_obj_get_style_margin_left(obj, LV_PART_MAIN)   == 0);
    TEST_CHECK(lv_obj_get_style_margin_right(obj, LV_PART_MAIN)  == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}