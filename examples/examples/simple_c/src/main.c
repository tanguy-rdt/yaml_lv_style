#include <lvgl.h>

#include <stdbool.h>
#include <unistd.h>

#include "styles/styles.h"
#include "styles/stylesheet_dark.h"
#include "styles/stylesheet_light.h"
#include "styles/stylesheets.h"

static bool is_dark = false;

static void button_event_cb(lv_event_t* e) {
    const lv_event_code_t code = lv_event_get_code(e);
    lv_obj_t* btn = lv_event_get_target(e);
    lv_obj_t* screen = lv_screen_active();

    if (code == LV_EVENT_CLICKED) {
        is_dark = !is_dark;

        if (is_dark) {
            set_dark_style(screen, DARK_STYLE_SCREEN);
            set_dark_style(btn, DARK_STYLE_BUTTON);
        } else {
            set_light_style(screen, LIGHT_STYLE_SCREEN);
            set_light_style(btn, LIGHT_STYLE_BUTTON);
        }
    }
}

int main() {
    lv_init();

    lv_disp_t* display = lv_sdl_window_create(300, 300);
    lv_indev_t* _mouse = lv_sdl_mouse_create();

    lv_obj_t* button = lv_button_create(lv_screen_active());
    lv_obj_t* label = lv_label_create(button);
    lv_label_set_text(label, "Switch Theme");

    init_style_sheets();
    set_light_style(lv_screen_active(), LIGHT_STYLE_SCREEN);
    set_light_style(button, LIGHT_STYLE_BUTTON);

    lv_obj_add_event_cb(button, button_event_cb, LV_EVENT_CLICKED, NULL);

    while (true) {
        uint32_t ms_delay = lv_timer_handler();
        usleep(ms_delay * 1000);
    }

    return 0;
}