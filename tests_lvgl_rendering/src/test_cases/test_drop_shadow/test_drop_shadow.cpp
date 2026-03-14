#include <lvgl.h>

#include "test_macros.h"
#include "lv_macros.h"

#include "styles_gen/stylesheets_macros.h"

LV_IMAGE_DECLARE(test_img_lvgl_logo_png);

static lv_obj_t* create_test_obj(lv_obj_t* parent) {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, LV_PCT(90), LV_PCT(90));
    lv_obj_center(obj);
    lv_obj_set_style_bg_image_src(obj, &test_img_lvgl_logo_png, LV_PART_MAIN);

    lv_obj_t* obj_blured = lv_obj_create(obj);
    lv_obj_set_size(obj_blured, 75, 75);
    lv_obj_center(obj_blured);

    return obj_blured;
}

TEST("test_drop_shadow_basic") {
    lv_obj_t* obj = create_test_obj(lv_screen_active());
    SET_DROP_SHADOW_STYLE(obj, ENUM_DROP_SHADOW_STYLE_TEST_DROP_SHADOW_BASIC);

    TEST_CHECK(lv_obj_get_style_drop_shadow_radius(obj, LV_PART_MAIN)   == 0);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_x(obj, LV_PART_MAIN) == 0);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_y(obj, LV_PART_MAIN) == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_drop_shadow_radius(obj, LV_PART_MAIN)   == 10);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_x(obj, LV_PART_MAIN) == 5);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_y(obj, LV_PART_MAIN) == 5);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_drop_shadow_radius(obj, LV_PART_MAIN)   == 40);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_x(obj, LV_PART_MAIN) == 10);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_y(obj, LV_PART_MAIN) == 10);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_drop_shadow_color") {
    lv_obj_t* obj = create_test_obj(lv_screen_active());
    SET_DROP_SHADOW_STYLE(obj, ENUM_DROP_SHADOW_STYLE_TEST_DROP_SHADOW_COLOR);

    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_drop_shadow_opa") {
    lv_obj_t* obj = create_test_obj(lv_screen_active());
    SET_DROP_SHADOW_STYLE(obj, ENUM_DROP_SHADOW_STYLE_TEST_DROP_SHADOW_OPA);

    TEST_CHECK(lv_obj_get_style_drop_shadow_opa(obj, LV_PART_MAIN) == LV_OPA_TRANSP);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_drop_shadow_opa(obj, LV_PART_MAIN) == LV_OPA_50);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_drop_shadow_opa(obj, LV_PART_MAIN) == LV_OPA_COVER);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_drop_shadow_quality") {
    lv_obj_t* obj = create_test_obj(lv_screen_active());
    SET_DROP_SHADOW_STYLE(obj, ENUM_DROP_SHADOW_STYLE_TEST_DROP_SHADOW_QUALITY);

    TEST_CHECK(lv_obj_get_style_drop_shadow_quality(obj, LV_PART_MAIN) == LV_BLUR_QUALITY_AUTO);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_drop_shadow_quality(obj, LV_PART_MAIN) == LV_BLUR_QUALITY_SPEED);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_drop_shadow_quality(obj, LV_PART_MAIN) == LV_BLUR_QUALITY_PRECISION);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_drop_shadow_offset") {
    lv_obj_t* obj = create_test_obj(lv_screen_active());
    SET_DROP_SHADOW_STYLE(obj, ENUM_DROP_SHADOW_STYLE_TEST_DROP_SHADOW_OFFSET);

    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_x(obj, LV_PART_MAIN) == 0);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_y(obj, LV_PART_MAIN) == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_x(obj, LV_PART_MAIN) == 20);
    TEST_CHECK(lv_obj_get_style_drop_shadow_offset_y(obj, LV_PART_MAIN) == 20);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}