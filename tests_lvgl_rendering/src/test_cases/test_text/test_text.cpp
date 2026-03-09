#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST("test_text_color") {
    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello World");
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_COLOR);

    TEST_CHECK(lv_obj_get_style_text_opa(label, LV_PART_MAIN) == LV_OPA_COVER);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_text_opa(label, LV_PART_MAIN) == LV_OPA_50);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_text_opa(label, LV_PART_MAIN) == LV_OPA_TRANSP);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_text_spacing") {
    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello\nWorld");
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_SPACING);

    TEST_CHECK(lv_obj_get_style_text_letter_space(label, LV_PART_MAIN) == 0);
    TEST_CHECK(lv_obj_get_style_text_line_space(label, LV_PART_MAIN)   == 0);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_text_letter_space(label, LV_PART_MAIN) == 5);
    TEST_CHECK(lv_obj_get_style_text_line_space(label, LV_PART_MAIN)   == 10);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_text_letter_space(label, LV_PART_MAIN) == 15);
    TEST_CHECK(lv_obj_get_style_text_line_space(label, LV_PART_MAIN)   == 25);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_text_decor") {
    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello World");
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_DECOR);

    TEST_CHECK(lv_obj_get_style_text_decor(label, LV_PART_MAIN) == LV_TEXT_DECOR_NONE);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_text_decor(label, LV_PART_MAIN) == LV_TEXT_DECOR_UNDERLINE);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_text_decor(label, LV_PART_MAIN) == LV_TEXT_DECOR_STRIKETHROUGH);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_text_align") {
    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello\nWorld\nTest");
    lv_obj_set_width(label, 200);
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_ALIGN);

    TEST_CHECK(lv_obj_get_style_text_align(label, LV_PART_MAIN) == LV_TEXT_ALIGN_LEFT);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_text_align(label, LV_PART_MAIN) == LV_TEXT_ALIGN_CENTER);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_text_align(label, LV_PART_MAIN) == LV_TEXT_ALIGN_RIGHT);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}

TEST("test_text_outline") {
    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello World");
    lv_obj_center(label);
    SET_TEXT_STYLE(label, ENUM_TEXT_STYLE_TEST_TEXT_OUTLINE);

    TEST_CHECK(lv_obj_get_style_text_outline_stroke_width(label, LV_PART_MAIN) == 2);
    TEST_CHECK(lv_obj_get_style_text_outline_stroke_opa(label, LV_PART_MAIN)   == LV_OPA_COVER);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_text_outline_stroke_width(label, LV_PART_MAIN) == 5);
    TEST_CHECK(lv_obj_get_style_text_outline_stroke_opa(label, LV_PART_MAIN)   == LV_OPA_50);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(label, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_text_outline_stroke_width(label, LV_PART_MAIN) == 0);
    TEST_CHECK(lv_obj_get_style_text_outline_stroke_opa(label, LV_PART_MAIN)   == LV_OPA_TRANSP);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}