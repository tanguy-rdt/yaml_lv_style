#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST("test_border_basic") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BORDER_STYLE(obj, ENUM_BORDER_STYLE_TEST_BORDER_BASIC);

    TEST_CHECK(lv_obj_get_style_border_opa(obj, LV_PART_MAIN)   == LV_OPA_COVER);
    TEST_CHECK(lv_obj_get_style_border_width(obj, LV_PART_MAIN) == 2);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_border_opa(obj, LV_PART_MAIN)   == LV_OPA_50);
    TEST_CHECK(lv_obj_get_style_border_width(obj, LV_PART_MAIN) == 5);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_border_opa(obj, LV_PART_MAIN)   == LV_OPA_TRANSP);
    TEST_CHECK(lv_obj_get_style_border_width(obj, LV_PART_MAIN) == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_border_side") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BORDER_STYLE(obj, ENUM_BORDER_STYLE_TEST_BORDER_SIDE);

    TEST_CHECK(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_FULL);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_TOP);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_BOTTOM);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_3);
    TEST_CHECK(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_LEFT);
    TEST_CHECK_SCREENSHOT_COMPARE("user_3");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_4);
    TEST_CHECK(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_RIGHT);
    TEST_CHECK_SCREENSHOT_COMPARE("user_4");
}

TEST("test_border_post") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BORDER_STYLE(obj, ENUM_BORDER_STYLE_TEST_BORDER_POST);

    lv_obj_t* child = lv_obj_create(obj);
    lv_obj_set_size(child, 150, 150);
    lv_obj_center(child);

    TEST_CHECK(lv_obj_get_style_border_post(obj, LV_PART_MAIN) == false);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_border_post(obj, LV_PART_MAIN) == true);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");
}