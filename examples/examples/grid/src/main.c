#include <lvgl.h>

#include <unistd.h>
#include <stdio.h>

#include "generated_styles/styles.h"
#include "generated_styles/stylesheets.h"
#include "generated_styles/stylesheet_grid.h"

int main() {
    lv_init();

    init_stylesheets();

    lv_disp_t* display = lv_sdl_window_create(300, 300);
    lv_indev_t* _mouse = lv_sdl_mouse_create();

    lv_obj_t* grid = lv_obj_create(lv_screen_active());
    set_grid_style(grid, GRID_STYLE_LAYOUT);

    const int styles[2][2] = {
        { GRID_STYLE_CELL_0_0, GRID_STYLE_CELL_1_0 },
        { GRID_STYLE_CELL_0_1, GRID_STYLE_CELL_1_1 }
    };

    for (int y = 0; y < 2; y++) {
        for (int x = 0; x < 2; x++) {
            lv_obj_t* obj = lv_obj_create(grid);
            set_grid_style(obj, styles[y][x]);

            lv_obj_t* label = lv_label_create(obj);
            char text[16];
            snprintf(text, sizeof(text), "cell_%d_%d", x, y);
            lv_label_set_text(label, text);
        }
    }

    while (true) {
        uint32_t ms_delay = lv_timer_handler();
        usleep(ms_delay * 1000);
    }

    return 0;
}