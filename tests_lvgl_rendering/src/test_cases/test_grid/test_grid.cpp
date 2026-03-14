#include <lvgl.h>

#include "test_macros.h"
#include "lv_macros.h"

#include "styles_gen/stylesheets_macros.h"

static lv_obj_t* create_grid_container(lv_obj_t* parent) {
    lv_obj_t* container = lv_obj_create(parent);
    lv_obj_set_size(container, 350, 350);
    lv_obj_center(container);
    return container;
}

TEST("test_grid_layout") {
    lv_obj_t* container = create_grid_container(lv_screen_active());
    SET_GRID_STYLE(container, ENUM_GRID_STYLE_TEST_GRID_LAYOUT);

    for (int i = 0; i < 6; i++) {
        lv_obj_t* child = lv_obj_create(container);
        lv_obj_set_size(child, 80, 80);
        lv_obj_set_style_grid_cell_column_pos(child, i % 3, LV_PART_MAIN);
        lv_obj_set_style_grid_cell_row_pos(child, i / 3, LV_PART_MAIN);
        lv_obj_set_style_grid_cell_column_span(child, 1, LV_PART_MAIN);
        lv_obj_set_style_grid_cell_row_span(child, 1, LV_PART_MAIN);
    }

    TEST_CHECK(lv_obj_get_style_grid_column_align(container, LV_PART_MAIN) == LV_GRID_ALIGN_START);
    TEST_CHECK(lv_obj_get_style_grid_row_align(container, LV_PART_MAIN)    == LV_GRID_ALIGN_START);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_grid_column_align(container, LV_PART_MAIN) == LV_GRID_ALIGN_CENTER);
    TEST_CHECK(lv_obj_get_style_grid_row_align(container, LV_PART_MAIN)    == LV_GRID_ALIGN_CENTER);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_grid_column_align(container, LV_PART_MAIN) == LV_GRID_ALIGN_END);
    TEST_CHECK(lv_obj_get_style_grid_row_align(container, LV_PART_MAIN)    == LV_GRID_ALIGN_END);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");

    lv_obj_clear_and_set_state(container, LV_STATE_USER_3);
    TEST_CHECK(lv_obj_get_style_grid_column_align(container, LV_PART_MAIN) == LV_GRID_ALIGN_SPACE_EVENLY);
    TEST_CHECK(lv_obj_get_style_grid_row_align(container, LV_PART_MAIN)    == LV_GRID_ALIGN_SPACE_BETWEEN);
    TEST_CHECK_SCREENSHOT_COMPARE("user_3");
}

TEST("test_grid_cell") {
    lv_obj_t* container = create_grid_container(lv_screen_active());

    lv_obj_t* obj = lv_obj_create(container);
    lv_obj_set_size(obj, 80, 80);
    SET_GRID_STYLE(obj, ENUM_GRID_STYLE_TEST_GRID_CELL);

    for (int i = 1; i < 6; i++) {
        lv_obj_t* child = lv_obj_create(container);
        lv_obj_set_size(child, 80, 80);
        lv_obj_set_style_grid_cell_column_pos(child, i % 3, LV_PART_MAIN);
        lv_obj_set_style_grid_cell_row_pos(child, i / 3, LV_PART_MAIN);
        lv_obj_set_style_grid_cell_column_span(child, 1, LV_PART_MAIN);
        lv_obj_set_style_grid_cell_row_span(child, 1, LV_PART_MAIN);
    }

    TEST_CHECK(lv_obj_get_style_grid_cell_column_pos(obj, LV_PART_MAIN)  == 0);
    TEST_CHECK(lv_obj_get_style_grid_cell_row_pos(obj, LV_PART_MAIN)     == 0);
    TEST_CHECK(lv_obj_get_style_grid_cell_column_span(obj, LV_PART_MAIN) == 1);
    TEST_CHECK(lv_obj_get_style_grid_cell_row_span(obj, LV_PART_MAIN)    == 1);
    TEST_CHECK(lv_obj_get_style_grid_cell_x_align(obj, LV_PART_MAIN)     == LV_GRID_ALIGN_START);
    TEST_CHECK(lv_obj_get_style_grid_cell_y_align(obj, LV_PART_MAIN)     == LV_GRID_ALIGN_START);
    TEST_CHECK_SCREENSHOT_COMPARE("default");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_1);
    TEST_CHECK(lv_obj_get_style_grid_cell_column_pos(obj, LV_PART_MAIN)  == 1);
    TEST_CHECK(lv_obj_get_style_grid_cell_row_pos(obj, LV_PART_MAIN)     == 1);
    TEST_CHECK(lv_obj_get_style_grid_cell_column_span(obj, LV_PART_MAIN) == 2);
    TEST_CHECK(lv_obj_get_style_grid_cell_row_span(obj, LV_PART_MAIN)    == 2);
    TEST_CHECK(lv_obj_get_style_grid_cell_x_align(obj, LV_PART_MAIN)     == LV_GRID_ALIGN_CENTER);
    TEST_CHECK(lv_obj_get_style_grid_cell_y_align(obj, LV_PART_MAIN)     == LV_GRID_ALIGN_CENTER);
    TEST_CHECK_SCREENSHOT_COMPARE("user_1");

    lv_obj_clear_and_set_state(obj, LV_STATE_USER_2);
    TEST_CHECK(lv_obj_get_style_grid_cell_column_pos(obj, LV_PART_MAIN)  == 0);
    TEST_CHECK(lv_obj_get_style_grid_cell_row_pos(obj, LV_PART_MAIN)     == 0);
    TEST_CHECK(lv_obj_get_style_grid_cell_column_span(obj, LV_PART_MAIN) == 1);
    TEST_CHECK(lv_obj_get_style_grid_cell_row_span(obj, LV_PART_MAIN)    == 1);
    TEST_CHECK(lv_obj_get_style_grid_cell_x_align(obj, LV_PART_MAIN)     == LV_GRID_ALIGN_END);
    TEST_CHECK(lv_obj_get_style_grid_cell_y_align(obj, LV_PART_MAIN)     == LV_GRID_ALIGN_END);
    TEST_CHECK_SCREENSHOT_COMPARE("user_2");
}