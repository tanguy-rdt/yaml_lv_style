#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_size") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    SET_SIZE_STYLE(obj, ENUM_SIZE_STYLE_TEST_SIZE);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_width(obj)                          == 75);
    REQUIRE(lv_obj_get_height(obj)                         == 75);
    REQUIRE(lv_obj_get_style_min_width(obj, LV_PART_MAIN)  == 50);
    REQUIRE(lv_obj_get_style_min_height(obj, LV_PART_MAIN) == 50);
    REQUIRE(lv_obj_get_style_max_width(obj, LV_PART_MAIN)  == 100);
    REQUIRE(lv_obj_get_style_max_height(obj, LV_PART_MAIN) == 100);

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_width(obj)  == 100);
    REQUIRE(lv_obj_get_height(obj) == 100);

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_width(obj)  == 50);
    REQUIRE(lv_obj_get_height(obj) == 50);
}

TEST_CASE("test_abs_pos") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    SET_POSITION_STYLE(obj, ENUM_POSITION_STYLE_TEST_ABS_POS);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_x(obj) == 400);
    REQUIRE(lv_obj_get_y(obj) == 200);

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_x(obj) == 0);
    REQUIRE(lv_obj_get_y(obj) == 0);
}

TEST_CASE("test_alignment") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    SET_POSITION_STYLE(obj, ENUM_POSITION_STYLE_TEST_ALIGNMENT);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_align(obj, LV_PART_MAIN) == LV_ALIGN_CENTER);

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_align(obj, LV_PART_MAIN) == LV_ALIGN_TOP_LEFT);

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_align(obj, LV_PART_MAIN) == LV_ALIGN_BOTTOM_RIGHT);
}

TEST_CASE("test_transform_size") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_center(obj);
    SET_TRANSFORM_STYLE(obj, ENUM_TRANSFORM_STYLE_TEST_TRANSFORM_SIZE);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_transform_scale") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_center(obj);
    SET_TRANSFORM_STYLE(obj, ENUM_TRANSFORM_STYLE_TEST_TRANSFORM_SCALE);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_transform_rotation") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_center(obj);
    SET_TRANSFORM_STYLE(obj, ENUM_TRANSFORM_STYLE_TEST_TRANSFORM_ROTATION);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_transform_pivot") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_center(obj);
    SET_TRANSFORM_STYLE(obj, ENUM_TRANSFORM_STYLE_TEST_TRANSFORM_PIVOT);

    lv_obj_set_style_transform_rotation(obj, 450, LV_PART_MAIN);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_1");
}

TEST_CASE("test_translate") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_center(obj);
    SET_TRANSLATE_STYLE(obj, ENUM_TRANSLATE_STYLE_TEST_TRANSLATE);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_transform_skew") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 600, 300);
    lv_obj_center(container);
    lv_obj_t* obj = lv_obj_create(container);
    SET_TRANSFORM_STYLE(obj, ENUM_TRANSFORM_STYLE_TEST_TRANSFORM_SKEW);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}