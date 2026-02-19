#include <lvgl.h>

#include <unistd.h>

#include "generated_styles/styles/styles.h"
#include "generated_styles/stylesheets/stylesheets.h"

static bool is_dark = false;

static void button_event_cb(lv_event_t * e) {
    const lv_event_code_t code = lv_event_get_code(e);
    auto * btn = static_cast<lv_obj_t*>(lv_event_get_target(e));
    lv_obj_t * screen = lv_screen_active();

    if(code == LV_EVENT_CLICKED) {
        is_dark = !is_dark;

        if(is_dark) {
            set_style(screen, DarkStyle::SCREEN);
            set_style(btn, DarkStyle::BUTTON);
        } else {
            set_style(screen, LightStyle::SCREEN);
            set_style(btn, LightStyle::BUTTON);
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

    init_stylesheets();
    set_style(lv_screen_active(), LightStyle::SCREEN);
    set_style(button, LightStyle::BUTTON);

    lv_obj_add_event_cb(button, button_event_cb, LV_EVENT_CLICKED, nullptr);

    while (true) {
        uint32_t ms_delay = lv_timer_handler();
        usleep(ms_delay * 1000);
    }

    return 0;
}