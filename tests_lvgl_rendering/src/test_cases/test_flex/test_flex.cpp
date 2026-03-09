#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

static lv_obj_t* create_flex_container(lv_obj_t* parent) {
    lv_obj_t* container = lv_obj_create(parent);
    lv_obj_set_size(container, 300, 300);
    lv_obj_center(container);
    lv_obj_set_style_layout(container, LV_LAYOUT_FLEX, LV_PART_MAIN);
    for (int i = 0; i < 4; i++) {
        lv_obj_t* child = lv_obj_create(container);
        lv_obj_set_size(child, 60, 60);
    }
    return container;
}

TEST("test_flex_flow") {
    lv_obj_t* container = create_flex_container(lv_screen_active());
    SET_FLEX_STYLE(container, ENUM_FLEX_STYLE_TEST_FLEX_FLOW);

    TEST_CHECK(lv_obj_get_style_flex_flow(container, LV_PART_MAIN) == LV_FLEX_FLOW_ROW);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_flex_flow(container, LV_PART_MAIN) == LV_FLEX_FLOW_COLUMN);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_flex_flow(container, LV_PART_MAIN) == LV_FLEX_FLOW_ROW_WRAP);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_3);
    TEST_CHECK(lv_obj_get_style_flex_flow(container, LV_PART_MAIN) == LV_FLEX_FLOW_COLUMN_WRAP);
    TEST_CHECK_SCREENSHOT_COMPARE("user_3");
}

TEST("test_flex_place") {
    lv_obj_t* container = create_flex_container(lv_screen_active());
    SET_FLEX_STYLE(container, ENUM_FLEX_STYLE_TEST_FLEX_PLACE);

    TEST_CHECK(lv_obj_get_style_flex_main_place(container, LV_PART_MAIN)  == LV_FLEX_ALIGN_START);
    TEST_CHECK(lv_obj_get_style_flex_cross_place(container, LV_PART_MAIN) == LV_FLEX_ALIGN_START);
    TEST_CHECK(lv_obj_get_style_flex_track_place(container, LV_PART_MAIN) == LV_FLEX_ALIGN_START);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_flex_main_place(container, LV_PART_MAIN)  == LV_FLEX_ALIGN_CENTER);
    TEST_CHECK(lv_obj_get_style_flex_cross_place(container, LV_PART_MAIN) == LV_FLEX_ALIGN_CENTER);
    TEST_CHECK(lv_obj_get_style_flex_track_place(container, LV_PART_MAIN) == LV_FLEX_ALIGN_CENTER);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_flex_main_place(container, LV_PART_MAIN)  == LV_FLEX_ALIGN_END);
    TEST_CHECK(lv_obj_get_style_flex_cross_place(container, LV_PART_MAIN) == LV_FLEX_ALIGN_END);
    TEST_CHECK(lv_obj_get_style_flex_track_place(container, LV_PART_MAIN) == LV_FLEX_ALIGN_END);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_3);
    TEST_CHECK(lv_obj_get_style_flex_main_place(container, LV_PART_MAIN)  == LV_FLEX_ALIGN_SPACE_EVENLY);
    TEST_CHECK(lv_obj_get_style_flex_cross_place(container, LV_PART_MAIN) == LV_FLEX_ALIGN_START);
    TEST_CHECK(lv_obj_get_style_flex_track_place(container, LV_PART_MAIN) == LV_FLEX_ALIGN_SPACE_BETWEEN);
    TEST_CHECK_SCREENSHOT_COMPARE("user_3");
}

TEST("test_flex_grow") {
    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 300, 100);
    lv_obj_center(container);
    lv_obj_set_style_layout(container, LV_LAYOUT_FLEX, LV_PART_MAIN);
    lv_obj_set_style_flex_flow(container, LV_FLEX_FLOW_ROW, LV_PART_MAIN);

    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_set_size(obj, 60, 60);
    SET_FLEX_STYLE(obj, ENUM_FLEX_STYLE_TEST_FLEX_GROW);

    lv_obj_t* child1 = lv_obj_create(container);
    lv_obj_set_size(child1, 60, 60);
    lv_obj_t* child2 = lv_obj_create(container);
    lv_obj_set_size(child2, 60, 60);

    TEST_CHECK(lv_obj_get_style_flex_grow(obj, LV_PART_MAIN) == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_flex_grow(obj, LV_PART_MAIN) == 1);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_flex_grow(obj, LV_PART_MAIN) == 3);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}