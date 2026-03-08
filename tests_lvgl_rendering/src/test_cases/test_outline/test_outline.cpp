#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_outline_basic") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    SET_OUTLINE_STYLE(obj, ENUM_OUTLINE_STYLE_TEST_OUTLINE_BASIC);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_outline_width(obj, LV_PART_MAIN) == 2);
    REQUIRE(lv_obj_get_style_outline_opa(obj, LV_PART_MAIN)   == LV_OPA_COVER);
    REQUIRE(lv_obj_get_style_outline_pad(obj, LV_PART_MAIN)   == 5);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_outline_width(obj, LV_PART_MAIN) == 5);
    REQUIRE(lv_obj_get_style_outline_opa(obj, LV_PART_MAIN)   == LV_OPA_50);
    REQUIRE(lv_obj_get_style_outline_pad(obj, LV_PART_MAIN)   == 15);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_outline_width(obj, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_outline_opa(obj, LV_PART_MAIN)   == LV_OPA_TRANSP);
    REQUIRE(lv_obj_get_style_outline_pad(obj, LV_PART_MAIN)   == 0);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}