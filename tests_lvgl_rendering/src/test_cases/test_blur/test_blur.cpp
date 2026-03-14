#include <lvgl.h>

#include "test_macros.h"
#include "lv_macros.h"

#include "styles_gen/stylesheets_macros.h"

LV_IMAGE_DECLARE(test_img_lvgl_logo_png);

static lv_obj_t* create_test_obj(lv_obj_t* parent) {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, LV_PCT(90), LV_PCT(90));
    lv_obj_center(obj);

    lv_obj_t* label = lv_label_create(obj);
    lv_label_set_text(label, "Hello, World !");
    lv_obj_center(label);
    lv_obj_set_style_text_font(label, &lv_font_montserrat_48, LV_PART_MAIN);

    lv_obj_t* obj_blured = lv_obj_create(obj);
    lv_obj_set_size(obj_blured, 200, 100);
    lv_obj_center(obj_blured);
    lv_obj_set_style_bg_image_src(obj_blured, &test_img_lvgl_logo_png, LV_PART_MAIN);

    return obj_blured;
}

TEST("test_blur_radius") {
    lv_obj_t* obj = create_test_obj(lv_screen_active());
    SET_BLUR_STYLE(obj, ENUM_BLUR_STYLE_TEST_BLUR_RADIUS);

    TEST_CHECK(lv_obj_get_style_blur_radius(obj, LV_PART_MAIN) == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_blur_radius(obj, LV_PART_MAIN) == 5);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_blur_radius(obj, LV_PART_MAIN) == 20);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_blur_backdrop") {
    lv_obj_t* obj = create_test_obj(lv_screen_active());
    SET_BLUR_STYLE(obj, ENUM_BLUR_STYLE_TEST_BLUR_BACKDROP);

    TEST_CHECK(lv_obj_get_style_blur_backdrop(obj, LV_PART_MAIN) == false);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_blur_backdrop(obj, LV_PART_MAIN) == true);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");
}

TEST("test_blur_quality") {
    lv_obj_t* obj = create_test_obj(lv_screen_active());
    SET_BLUR_STYLE(obj, ENUM_BLUR_STYLE_TEST_BLUR_QUALITY);

    TEST_CHECK(lv_obj_get_style_blur_quality(obj, LV_PART_MAIN) == LV_BLUR_QUALITY_AUTO);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_blur_quality(obj, LV_PART_MAIN) == LV_BLUR_QUALITY_SPEED);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_blur_quality(obj, LV_PART_MAIN) == LV_BLUR_QUALITY_PRECISION);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}