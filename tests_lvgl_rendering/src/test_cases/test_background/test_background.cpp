#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

LV_IMAGE_DECLARE(test_img_lvgl_logo_png);

TEST("test_bg_color") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BACKGROUND_STYLE(obj, ENUM_BACKGROUND_STYLE_TEST_BG_COLOR);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    TEST_CHECK(lv_obj_get_style_bg_opa(obj, LV_PART_MAIN) == LV_OPA_COVER);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_bg_opa(obj, LV_PART_MAIN) == LV_OPA_50);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_bg_opa(obj, LV_PART_MAIN) == LV_OPA_TRANSP);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_bg_grad") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BACKGROUND_STYLE(obj, ENUM_BACKGROUND_STYLE_TEST_BG_GRAD);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    TEST_CHECK(lv_obj_get_style_bg_grad_dir(obj, LV_PART_MAIN)  == LV_GRAD_DIR_HOR);
    TEST_CHECK(lv_obj_get_style_bg_main_stop(obj, LV_PART_MAIN) == 0);
    TEST_CHECK(lv_obj_get_style_bg_grad_stop(obj, LV_PART_MAIN) == 255);
    TEST_CHECK(lv_obj_get_style_bg_main_opa(obj, LV_PART_MAIN)  == 255);
    TEST_CHECK(lv_obj_get_style_bg_grad_opa(obj, LV_PART_MAIN)  == 255);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_bg_grad_dir(obj, LV_PART_MAIN)  == LV_GRAD_DIR_VER);
    TEST_CHECK(lv_obj_get_style_bg_main_stop(obj, LV_PART_MAIN) == 64);
    TEST_CHECK(lv_obj_get_style_bg_grad_stop(obj, LV_PART_MAIN) == 192);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_bg_grad_dir(obj, LV_PART_MAIN)  == LV_GRAD_DIR_NONE);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_bg_image") {
    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    lv_obj_set_style_bg_image_src(obj, &test_img_lvgl_logo_png, LV_PART_MAIN);
    SET_BACKGROUND_STYLE(obj, ENUM_BACKGROUND_STYLE_TEST_BG_IMAGE);

    TEST_CHECK(lv_obj_get_style_bg_image_opa(obj, LV_PART_MAIN)         == LV_OPA_COVER);
    TEST_CHECK(lv_obj_get_style_bg_image_recolor_opa(obj, LV_PART_MAIN) == LV_OPA_TRANSP);
    TEST_CHECK(lv_obj_get_style_bg_image_tiled(obj, LV_PART_MAIN)       == false);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_set_state(obj, LV_STATE_DEFAULT, false);
    lv_obj_set_state(obj, LV_STATE_USER_1, true);
    TEST_CHECK(lv_obj_get_style_bg_image_opa(obj, LV_PART_MAIN)         == LV_OPA_50);
    TEST_CHECK(lv_obj_get_style_bg_image_recolor_opa(obj, LV_PART_MAIN) == LV_OPA_50);
    TEST_CHECK(lv_obj_get_style_bg_image_tiled(obj, LV_PART_MAIN)       == true);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");
}