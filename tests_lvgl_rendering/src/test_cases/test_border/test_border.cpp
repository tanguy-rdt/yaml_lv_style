#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_border_basic") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BORDER_STYLE(obj, ENUM_BORDER_STYLE_TEST_BORDER_BASIC);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_opa(obj, LV_PART_MAIN)   == LV_OPA_COVER);
    REQUIRE(lv_obj_get_style_border_width(obj, LV_PART_MAIN) == 2);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_opa(obj, LV_PART_MAIN)   == LV_OPA_50);
    REQUIRE(lv_obj_get_style_border_width(obj, LV_PART_MAIN) == 5);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_opa(obj, LV_PART_MAIN)   == LV_OPA_TRANSP);
    REQUIRE(lv_obj_get_style_border_width(obj, LV_PART_MAIN) == 0);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_border_side") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BORDER_STYLE(obj, ENUM_BORDER_STYLE_TEST_BORDER_SIDE);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_FULL);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_TOP);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_BOTTOM);
    REQUIRE_SCREENSHOT_COMPARE("user_2");

    lv_obj_add_state(obj, LV_STATE_USER_3);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_LEFT);
    REQUIRE_SCREENSHOT_COMPARE("user_3");

    lv_obj_add_state(obj, LV_STATE_USER_4);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_side(obj, LV_PART_MAIN) == LV_BORDER_SIDE_RIGHT);
    REQUIRE_SCREENSHOT_COMPARE("user_4");
}

TEST_CASE("test_border_post") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BORDER_STYLE(obj, ENUM_BORDER_STYLE_TEST_BORDER_POST);

    lv_obj_t* child = lv_obj_create(obj);
    lv_obj_set_size(child, 150, 150);
    lv_obj_center(child);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_post(obj, LV_PART_MAIN) == false);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_border_post(obj, LV_PART_MAIN) == true);
    REQUIRE_SCREENSHOT_COMPARE("user_1");
}