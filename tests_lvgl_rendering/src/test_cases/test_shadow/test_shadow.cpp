#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_shadow_basic") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    SET_SHADOW_STYLE(obj, ENUM_SHADOW_STYLE_TEST_SHADOW_BASIC);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_width(obj, LV_PART_MAIN)    == 10);
    REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 5);
    REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 5);
    REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN)   == 0);
    REQUIRE(lv_obj_get_style_shadow_opa(obj, LV_PART_MAIN)      == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_width(obj, LV_PART_MAIN)    == 20);
    REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 10);
    REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 10);
    REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN)   == 5);
    REQUIRE(lv_obj_get_style_shadow_opa(obj, LV_PART_MAIN)      == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_width(obj, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_shadow_opa(obj, LV_PART_MAIN)   == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_shadow_spread") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    SET_SHADOW_STYLE(obj, ENUM_SHADOW_STYLE_TEST_SHADOW_SPREAD);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN) == 0);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN) == 10);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_spread(obj, LV_PART_MAIN) == 20);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_shadow_offset") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    SET_SHADOW_STYLE(obj, ENUM_SHADOW_STYLE_TEST_SHADOW_OFFSET);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 0);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 20);
    REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 0);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == 20);
    REQUIRE_SCREENSHOT_COMPARE("user_2");

    lv_obj_add_state(obj, LV_STATE_USER_3);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_shadow_offset_x(obj, LV_PART_MAIN) == -20);
    REQUIRE(lv_obj_get_style_shadow_offset_y(obj, LV_PART_MAIN) == -20);
    REQUIRE_SCREENSHOT_COMPARE("user_3");
}