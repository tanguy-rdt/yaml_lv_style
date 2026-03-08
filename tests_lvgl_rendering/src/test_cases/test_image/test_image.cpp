#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_image_opa") {
    INIT_STYLE_SHEETS();

    lv_obj_t* img = lv_image_create(lv_screen_active());
    lv_image_set_src(img, "path/to/image.png");
    lv_obj_center(img);
    SET_IMAGE_STYLE(img, ENUM_IMAGE_STYLE_TEST_IMAGE_OPA);

    lv_obj_add_state(img, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_image_opa(img, LV_PART_MAIN) == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(img, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_image_opa(img, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(img, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_image_opa(img, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_image_recolor") {
    INIT_STYLE_SHEETS();

    lv_obj_t* img = lv_image_create(lv_screen_active());
    lv_image_set_src(img, "path/to/image.png");
    lv_obj_center(img);
    SET_IMAGE_STYLE(img, ENUM_IMAGE_STYLE_TEST_IMAGE_RECOLOR);

    lv_obj_add_state(img, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_image_recolor_opa(img, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(img, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_image_recolor_opa(img, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(img, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_image_recolor_opa(img, LV_PART_MAIN) == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_image_colorkey") {
    INIT_STYLE_SHEETS();

    lv_obj_t* img = lv_image_create(lv_screen_active());
    lv_image_set_src(img, "path/to/image.png");
    lv_obj_center(img);
    SET_IMAGE_STYLE(img, ENUM_IMAGE_STYLE_TEST_IMAGE_COLORKEY);

    lv_obj_add_state(img, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(img, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE_SCREENSHOT_COMPARE("user_1");
}