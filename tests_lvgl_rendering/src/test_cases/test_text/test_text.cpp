#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_text_color") {
    INIT_STYLE_SHEETS();

    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello World");
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_COLOR);

    lv_obj_add_state(label, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_opa(label, LV_PART_MAIN) == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(label, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_opa(label, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(label, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_opa(label, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_text_spacing") {
    INIT_STYLE_SHEETS();

    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello\nWorld");
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_SPACING);

    lv_obj_add_state(label, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_letter_space(label, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_text_line_space(label, LV_PART_MAIN)   == 0);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(label, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_letter_space(label, LV_PART_MAIN) == 5);
    REQUIRE(lv_obj_get_style_text_line_space(label, LV_PART_MAIN)   == 10);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(label, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_letter_space(label, LV_PART_MAIN) == 15);
    REQUIRE(lv_obj_get_style_text_line_space(label, LV_PART_MAIN)   == 25);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_text_decor") {
    INIT_STYLE_SHEETS();

    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello World");
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_DECOR);

    lv_obj_add_state(label, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_decor(label, LV_PART_MAIN) == LV_TEXT_DECOR_NONE);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(label, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_decor(label, LV_PART_MAIN) == LV_TEXT_DECOR_UNDERLINE);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(label, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_decor(label, LV_PART_MAIN) == LV_TEXT_DECOR_STRIKETHROUGH);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_text_align") {
    INIT_STYLE_SHEETS();

    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello\nWorld\nTest");
    lv_obj_set_width(label, 200);
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_ALIGN);

    lv_obj_add_state(label, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_align(label, LV_PART_MAIN) == LV_TEXT_ALIGN_LEFT);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(label, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_align(label, LV_PART_MAIN) == LV_TEXT_ALIGN_CENTER);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(label, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_align(label, LV_PART_MAIN) == LV_TEXT_ALIGN_RIGHT);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_text_outline") {
    INIT_STYLE_SHEETS();

    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello World");
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_OUTLINE);

    lv_obj_add_state(label, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_outline_stroke_width(label, LV_PART_MAIN) == 2);
    REQUIRE(lv_obj_get_style_text_outline_stroke_opa(label, LV_PART_MAIN)   == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(label, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_outline_stroke_width(label, LV_PART_MAIN) == 5);
    REQUIRE(lv_obj_get_style_text_outline_stroke_opa(label, LV_PART_MAIN)   == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(label, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_text_outline_stroke_width(label, LV_PART_MAIN) == 0);
    REQUIRE(lv_obj_get_style_text_outline_stroke_opa(label, LV_PART_MAIN)   == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}