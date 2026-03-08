#include <lvgl.h>

#include "macros.h"
#include "styles_gen/stylesheets_macros.h"

TEST_CASE("test_radius") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_MISC_STYLE(obj, ENUM_MISC_STYLE_TEST_RADIUS);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_radius(obj, LV_PART_MAIN) == 0);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_radius(obj, LV_PART_MAIN) == 10);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_radius(obj, LV_PART_MAIN) == 50);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_clip_corner") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_MISC_STYLE(obj, ENUM_MISC_STYLE_TEST_CLIP_CORNER);

    lv_obj_t* child = lv_obj_create(obj);
    lv_obj_set_size(child, 250, 250);
    lv_obj_center(child);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_clip_corner(obj, LV_PART_MAIN) == false);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_clip_corner(obj, LV_PART_MAIN) == true);
    REQUIRE_SCREENSHOT_COMPARE("user_1");
}

TEST_CASE("test_opa") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    lv_obj_set_style_bg_color(obj, lv_color_hex(0xFF0000), LV_PART_MAIN);
    lv_obj_set_style_bg_opa(obj, LV_OPA_COVER, LV_PART_MAIN);
    SET_MISC_STYLE(obj, ENUM_MISC_STYLE_TEST_OPA);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_opa(obj, LV_PART_MAIN) == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_opa(obj, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_opa(obj, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_opa_layered") {
    INIT_STYLE_SHEETS();

    lv_obj_t* container = lv_obj_create(lv_screen_active());
    lv_obj_set_size(container, 200, 200);
    lv_obj_center(container);
    lv_obj_set_style_bg_color(container, lv_color_hex(0xFF0000), LV_PART_MAIN);
    lv_obj_set_style_bg_opa(container, LV_OPA_COVER, LV_PART_MAIN);
    SET_MISC_STYLE(container, ENUM_MISC_STYLE_TEST_OPA_LAYERED);

    lv_obj_t* child = lv_obj_create(container);
    lv_obj_set_size(child, 100, 100);
    lv_obj_center(child);
    lv_obj_set_style_bg_color(child, lv_color_hex(0x0000FF), LV_PART_MAIN);

    lv_obj_add_state(container, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_opa_layered(container, LV_PART_MAIN) == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(container, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_opa_layered(container, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(container, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_opa_layered(container, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_color_filter") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    lv_obj_set_style_bg_color(obj, lv_color_hex(0xFF0000), LV_PART_MAIN);
    lv_obj_set_style_bg_opa(obj, LV_OPA_COVER, LV_PART_MAIN);
    SET_MISC_STYLE(obj, ENUM_MISC_STYLE_TEST_COLOR_FILTER);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_color_filter_opa(obj, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_color_filter_opa(obj, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_color_filter_opa(obj, LV_PART_MAIN) == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_recolor") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    lv_obj_set_style_bg_color(obj, lv_color_hex(0xFFFFFF), LV_PART_MAIN);
    lv_obj_set_style_bg_opa(obj, LV_OPA_COVER, LV_PART_MAIN);
    SET_MISC_STYLE(obj, ENUM_MISC_STYLE_TEST_RECOLOR);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_recolor_opa(obj, LV_PART_MAIN) == LV_OPA_TRANSP);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_recolor_opa(obj, LV_PART_MAIN) == LV_OPA_50);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_recolor_opa(obj, LV_PART_MAIN) == LV_OPA_COVER);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_blend_mode") {
    INIT_STYLE_SHEETS();

    lv_obj_t* bg = lv_obj_create(lv_screen_active());
    lv_obj_set_size(bg, 300, 300);
    lv_obj_center(bg);
    lv_obj_set_style_bg_color(bg, lv_color_hex(0xFF0000), LV_PART_MAIN);
    lv_obj_set_style_bg_opa(bg, LV_OPA_COVER, LV_PART_MAIN);

    lv_obj_t* obj = lv_obj_create(bg);
    lv_obj_set_size(obj, 150, 150);
    lv_obj_center(obj);
    lv_obj_set_style_bg_color(obj, lv_color_hex(0x00FF00), LV_PART_MAIN);
    lv_obj_set_style_bg_opa(obj, LV_OPA_COVER, LV_PART_MAIN);
    SET_MISC_STYLE(obj, ENUM_MISC_STYLE_TEST_BLEND_MODE);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_blend_mode(obj, LV_PART_MAIN) == LV_BLEND_MODE_NORMAL);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_blend_mode(obj, LV_PART_MAIN) == LV_BLEND_MODE_ADDITIVE);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_blend_mode(obj, LV_PART_MAIN) == LV_BLEND_MODE_SUBTRACTIVE);
    REQUIRE_SCREENSHOT_COMPARE("user_2");

    lv_obj_add_state(obj, LV_STATE_USER_3);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_blend_mode(obj, LV_PART_MAIN) == LV_BLEND_MODE_MULTIPLY);
    REQUIRE_SCREENSHOT_COMPARE("user_3");
}

TEST_CASE("test_layout") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 300, 300);
    lv_obj_center(obj);
    SET_MISC_STYLE(obj, ENUM_MISC_STYLE_TEST_LAYOUT);

    for (int i = 0; i < 4; i++) {
        lv_obj_t* child = lv_obj_create(obj);
        lv_obj_set_size(child, 80, 80);
    }

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_layout(obj, LV_PART_MAIN) == LV_LAYOUT_NONE);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_layout(obj, LV_PART_MAIN) == LV_LAYOUT_FLEX);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_layout(obj, LV_PART_MAIN) == LV_LAYOUT_GRID);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_base_dir") {
    INIT_STYLE_SHEETS();

    lv_obj_t* label = lv_label_create(lv_screen_active());
    lv_label_set_text(label, "Hello World");
    lv_obj_set_width(label, 200);
    lv_obj_center(label);
    SET_MISC_STYLE(label, ENUM_MISC_STYLE_TEST_BASE_DIR);

    lv_obj_add_state(label, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_base_dir(label, LV_PART_MAIN) == LV_BASE_DIR_LTR);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(label, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_base_dir(label, LV_PART_MAIN) == LV_BASE_DIR_RTL);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(label, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_base_dir(label, LV_PART_MAIN) == LV_BASE_DIR_AUTO);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_radial_offset") {
    INIT_STYLE_SHEETS();

    lv_obj_t* scale = lv_scale_create(lv_screen_active());
    lv_obj_set_size(scale, 200, 200);
    lv_obj_center(scale);
    lv_scale_set_mode(scale, LV_SCALE_MODE_ROUND_INNER);
    SET_MISC_STYLE(scale, ENUM_MISC_STYLE_TEST_RADIAL_OFFSET);

    lv_obj_add_state(scale, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_radial_offset(scale, LV_PART_MAIN) == 0);
    REQUIRE_SCREENSHOT_COMPARE("default");

    lv_obj_add_state(scale, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_radial_offset(scale, LV_PART_MAIN) == 20);
    REQUIRE_SCREENSHOT_COMPARE("user_1");

    lv_obj_add_state(scale, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_radial_offset(scale, LV_PART_MAIN) == 50);
    REQUIRE_SCREENSHOT_COMPARE("user_2");
}

TEST_CASE("test_rotary_sensitivity") {
    INIT_STYLE_SHEETS();

    lv_obj_t* obj = lv_obj_create(lv_screen_active());
    lv_obj_set_size(obj, 200, 200);
    lv_obj_center(obj);
    SET_MISC_STYLE(obj, ENUM_MISC_STYLE_TEST_ROTARY_SENSITIVITY);

    lv_obj_add_state(obj, LV_STATE_DEFAULT);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_rotary_sensitivity(obj, LV_PART_MAIN) == 128);

    lv_obj_add_state(obj, LV_STATE_USER_1);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_rotary_sensitivity(obj, LV_PART_MAIN) == 64);

    lv_obj_add_state(obj, LV_STATE_USER_2);
    lv_refr_now(nullptr);
    REQUIRE(lv_obj_get_style_rotary_sensitivity(obj, LV_PART_MAIN) == 255);
}