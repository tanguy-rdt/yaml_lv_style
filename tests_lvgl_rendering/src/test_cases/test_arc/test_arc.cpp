#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST("test_arc_basic") {
    lv_obj_t* arc = lv_arc_create(lv_screen_active());
    lv_obj_set_size(arc, 200, 200);
    lv_obj_center(arc);
    lv_arc_set_range(arc, 0, 100);
    lv_arc_set_value(arc, 50);
    SET_ARC_STYLE(arc, ENUM_ARC_STYLE_TEST_ARC_BASIC);

    lv_obj_add_state(arc, LV_STATE_DEFAULT);
    TEST_REQUIRE(lv_obj_get_style_arc_width(arc, LV_PART_MAIN)   == 5);
    TEST_REQUIRE(lv_obj_get_style_arc_opa(arc, LV_PART_MAIN)     == LV_OPA_COVER);
    TEST_REQUIRE(lv_obj_get_style_arc_rounded(arc, LV_PART_MAIN) == false);
    TEST_REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(arc, LV_STATE_USER_1);
    TEST_REQUIRE(lv_obj_get_style_arc_width(arc, LV_PART_MAIN)   == 15);
    TEST_REQUIRE(lv_obj_get_style_arc_opa(arc, LV_PART_MAIN)     == LV_OPA_50);
    TEST_REQUIRE(lv_obj_get_style_arc_rounded(arc, LV_PART_MAIN) == true);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(arc, LV_STATE_USER_2);
    TEST_REQUIRE(lv_obj_get_style_arc_width(arc, LV_PART_MAIN) == 2);
    TEST_REQUIRE(lv_obj_get_style_arc_opa(arc, LV_PART_MAIN)   == LV_OPA_TRANSP);
    TEST_REQUIRE_SCREENSHOT_COMPARE("user_2");
}