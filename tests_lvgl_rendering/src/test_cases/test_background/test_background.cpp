#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_bg_color") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BACKGROUND_STYLE(obj, ENUM_BACKGROUND_STYLE_TEST_BG_COLOR);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_bg_opa(obj, LV_PART_MAIN) == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_bg_opa(obj, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_bg_opa(obj, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_bg_grad") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BACKGROUND_STYLE(obj, ENUM_BACKGROUND_STYLE_TEST_BG_GRAD);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_bg_grad_dir(obj, LV_PART_MAIN)  == LV_GRAD_DIR_HOR);
    REQUIRE(lv_obj_get_style_bg_main_stop(obj, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_bg_grad_stop(obj, LV_PART_MAIN) == 255);
    REQUIRE(lv_obj_get_style_bg_main_opa(obj, LV_PART_MAIN)  == 255);
    REQUIRE(lv_obj_get_style_bg_grad_opa(obj, LV_PART_MAIN)  == 255);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_bg_grad_dir(obj, LV_PART_MAIN)  == LV_GRAD_DIR_VER);
    REQUIRE(lv_obj_get_style_bg_main_stop(obj, LV_PART_MAIN) == 64);
    REQUIRE(lv_obj_get_style_bg_grad_stop(obj, LV_PART_MAIN) == 192);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_bg_grad_dir(obj, LV_PART_MAIN)  == LV_GRAD_DIR_NONE);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_bg_image") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_BACKGROUND_STYLE(obj, ENUM_BACKGROUND_STYLE_TEST_BG_IMAGE);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_bg_image_opa(obj, LV_PART_MAIN)         == LV_OPA_COVER);
    REQUIRE(lv_obj_get_style_bg_image_recolor_opa(obj, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE(lv_obj_get_style_bg_image_tiled(obj, LV_PART_MAIN)       == false);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_bg_image_opa(obj, LV_PART_MAIN)         == LV_OPA_50);
    REQUIRE(lv_obj_get_style_bg_image_recolor_opa(obj, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE(lv_obj_get_style_bg_image_tiled(obj, LV_PART_MAIN)       == true);
    REQUIRE_SCREENSHOT_COMPARE("user_1");
}