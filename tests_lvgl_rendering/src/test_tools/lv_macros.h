#ifndef LV_MACRO_H
#define LV_MACRO_H

#include <lvgl.h>

#define lv_obj_clear_and_set_state(obj, state)        \
    do {                                              \
    lv_obj_remove_state(obj, LV_STATE_ANY);           \
    lv_obj_add_state(obj, state);                     \
    } while(false)


#endif //LV_MACRO_H