#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST("test_shadow_basic") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    SET_SHADOW_STYLE(obj, ENUM_SHADOW_STYLE_TEST_SHADOW_BASIC);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    TEST_REQUIRE(lv_obj_get_style_shadow_width(obj, LV_PART_MAIN)    == 10);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 5);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 5);
    TEST_REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN)   == 0);
    TEST_REQUIRE(lv_obj_get_style_shadow_opa(obj, LV_PART_MAIN)      == LV_OPA_COVER);
    TEST_REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    TEST_REQUIRE(lv_obj_get_style_shadow_width(obj, LV_PART_MAIN)    == 20);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 10);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 10);
    TEST_REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN)   == 5);
    TEST_REQUIRE(lv_obj_get_style_shadow_opa(obj, LV_PART_MAIN)      == LV_OPA_50);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    TEST_REQUIRE(lv_obj_get_style_shadow_width(obj, LV_PART_MAIN) == 0);
    TEST_REQUIRE(lv_obj_get_style_shadow_opa(obj, LV_PART_MAIN)   == LV_OPA_TRANSP);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST("test_shadow_spread") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    SET_SHADOW_STYLE(obj, ENUM_SHADOW_STYLE_TEST_SHADOW_SPREAD);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    TEST_REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN) == 0);
    TEST_REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    TEST_REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN) == 10);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    TEST_REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN) == 20);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST("test_shadow_offset") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    SET_SHADOW_STYLE(obj, ENUM_SHADOW_STYLE_TEST_SHADOW_OFFSET);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 0);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 0);
    TEST_REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 20);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 0);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 0);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 20);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_2");

    lv_obj_add_state(obj, LV_STATE_USER_3);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == -20);
    TEST_REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == -20);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_3");
}