#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_margin") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 400, 400);
    lv_obj_center(container);
    lv_obj_set_flex_flow(container, LV_FLEX_FLOW_ROW_WRAP);

    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_set_size(obj, 100, 100);
    SET_MARGIN_STYLE(obj, ENUM_MARGIN_STYLE_TEST_MARGIN);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_margin_top(obj, LV_PART_MAIN)    == 10);
    REQUIRE(lv_obj_get_style_margin_bottom(obj, LV_PART_MAIN) == 10);
    REQUIRE(lv_obj_get_style_margin_left(obj, LV_PART_MAIN)   == 10);
    REQUIRE(lv_obj_get_style_margin_right(obj, LV_PART_MAIN)  == 10);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_margin_top(obj, LV_PART_MAIN)    == 30);
    REQUIRE(lv_obj_get_style_margin_bottom(obj, LV_PART_MAIN) == 30);
    REQUIRE(lv_obj_get_style_margin_left(obj, LV_PART_MAIN)   == 30);
    REQUIRE(lv_obj_get_style_margin_right(obj, LV_PART_MAIN)  == 30);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_margin_top(obj, LV_PART_MAIN)    == 0);
    REQUIRE(lv_obj_get_style_margin_bottom(obj, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_margin_left(obj, LV_PART_MAIN)   == 0);
    REQUIRE(lv_obj_get_style_margin_right(obj, LV_PART_MAIN)  == 0);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}