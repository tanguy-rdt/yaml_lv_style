#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST("test_pad") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_PADDING_STYLE(obj, ENUM_PADDING_STYLE_TEST_PAD);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    TEST_REQUIRE(lv_obj_get_style_pad_top(obj, LV_PART_MAIN)    == 10);
    TEST_REQUIRE(lv_obj_get_style_pad_bottom(obj, LV_PART_MAIN) == 10);
    TEST_REQUIRE(lv_obj_get_style_pad_left(obj, LV_PART_MAIN)   == 10);
    TEST_REQUIRE(lv_obj_get_style_pad_right(obj, LV_PART_MAIN)  == 10);

    lv_obj_add_state(obj, LV_STATE_USER_1);
    TEST_REQUIRE(lv_obj_get_style_pad_top(obj, LV_PART_MAIN)    == 30);
    TEST_REQUIRE(lv_obj_get_style_pad_bottom(obj, LV_PART_MAIN) == 30);
    TEST_REQUIRE(lv_obj_get_style_pad_left(obj, LV_PART_MAIN)   == 30);
    TEST_REQUIRE(lv_obj_get_style_pad_right(obj, LV_PART_MAIN)  == 30);

    lv_obj_add_state(obj, LV_STATE_USER_2);
    TEST_REQUIRE(lv_obj_get_style_pad_top(obj, LV_PART_MAIN)    == 0);
    TEST_REQUIRE(lv_obj_get_style_pad_bottom(obj, LV_PART_MAIN) == 0);
    TEST_REQUIRE(lv_obj_get_style_pad_left(obj, LV_PART_MAIN)   == 0);
    TEST_REQUIRE(lv_obj_get_style_pad_right(obj, LV_PART_MAIN)  == 0);
}

TEST("test_pad_radial") {
    lv_obj_t* scale = lv_scale_create(lv_screen_active());
    lv_obj_set_size(scale, 200, 200);
    lv_obj_center(scale);
    lv_scale_set_mode(scale, LV_SCALE_MODE_ROUND_INNER);
    SET_PADDING_STYLE(scale, ENUM_PADDING_STYLE_TEST_PAD_RADIAL);

    lv_obj_add_state(scale, LV_STATE_DEFAULT);
    TEST_REQUIRE(lv_obj_get_style_pad_radial(scale, LV_PART_INDICATOR) == 0);
    TEST_REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(scale, LV_STATE_USER_1);
    TEST_REQUIRE(lv_obj_get_style_pad_radial(scale, LV_PART_INDICATOR) == 20);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(scale, LV_STATE_USER_2);
    TEST_REQUIRE(lv_obj_get_style_pad_radial(scale, LV_PART_INDICATOR) == 50);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_2");
}