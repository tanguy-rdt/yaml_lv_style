#include <lvgl.h>

#include "test_macros.h"
#include "lv_macros.h"

#include "styles_gen/stylesheets_macros.h"

TEST("test_outline_basic") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    SET_OUTLINE_STYLE(obj, ENUM_OUTLINE_STYLE_TEST_OUTLINE_BASIC);

    TEST_CHECK(lv_obj_get_style_outline_width(obj, LV_PART_MAIN) == 2);
    TEST_CHECK(lv_obj_get_style_outline_opa(obj, LV_PART_MAIN)   == LV_OPA_COVER);
    TEST_CHECK(lv_obj_get_style_outline_pad(obj, LV_PART_MAIN)   == 5);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_outline_width(obj, LV_PART_MAIN) == 5);
    TEST_CHECK(lv_obj_get_style_outline_opa(obj, LV_PART_MAIN)   == LV_OPA_50);
    TEST_CHECK(lv_obj_get_style_outline_pad(obj, LV_PART_MAIN)   == 15);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_outline_width(obj, LV_PART_MAIN) == 0);
    TEST_CHECK(lv_obj_get_style_outline_opa(obj, LV_PART_MAIN)   == LV_OPA_TRANSP);
    TEST_CHECK(lv_obj_get_style_outline_pad(obj, LV_PART_MAIN)   == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}