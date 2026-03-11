#include <lvgl.h>

#include "test_macros.h"
#include "lv_macros.h"

#include "styles_gen/stylesheets_macros.h"

LV_IMAGE_DECLARE(test_img_lvgl_logo_png);

static lv_obj_t * img_create() {
    lv_obj_t* img = lv_image_create(lv_screen_active());
    lv_image_set_src(img, &test_img_lvgl_logo_png);
    return img;
}

TEST("test_image_opa") {
    lv_obj_t* img = img_create();
    lv_obj_center(img);
    SET_IMAGE_STYLE(img, ENUM_IMAGE_STYLE_TEST_IMAGE_OPA);

    TEST_CHECK(lv_obj_get_style_image_opa(img, LV_PART_MAIN) == LV_OPA_COVER);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(img, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_image_opa(img, LV_PART_MAIN) == LV_OPA_50);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");


    lv_obj_clear_and_set_state(img, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_image_opa(img, LV_PART_MAIN) == LV_OPA_TRANSP);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_image_recolor") {
    lv_obj_t* img = img_create();
    lv_obj_center(img);
    SET_IMAGE_STYLE(img, ENUM_IMAGE_STYLE_TEST_IMAGE_RECOLOR);

    TEST_CHECK(lv_obj_get_style_image_recolor_opa(img, LV_PART_MAIN) == LV_OPA_TRANSP);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(img, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_image_recolor_opa(img, LV_PART_MAIN) == LV_OPA_50);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(img, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_image_recolor_opa(img, LV_PART_MAIN) == LV_OPA_COVER);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_image_colorkey") {
    lv_obj_t* img = img_create();
    lv_obj_center(img);
    SET_IMAGE_STYLE(img, ENUM_IMAGE_STYLE_TEST_IMAGE_COLORKEY);

    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(img, LV_STATE_USER_1);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");
}