mod case_converters;
mod style_extractor;
mod style_info;

pub fn apply_filters(tera: &mut tera::Tera) {
    tera.register_filter("pascal_case", case_converters::pascal_case_filter);
    tera.register_filter(
        "screaming_snake_case",
        case_converters::screaming_snake_case_filter,
    );
    tera.register_filter("snake_case", case_converters::snake_case_filter);
    tera.register_filter("get_states_of_style", style_extractor::get_states_of_style);
    tera.register_filter(
        "get_props_by_states_sorted",
        style_extractor::get_props_by_states_sorted,
    );
    tera.register_filter("get_props_sorted", style_extractor::get_props_sorted);
    tera.register_filter("get_lv_state", style_extractor::get_lv_state);
    tera.register_filter("has_const_style", style_info::has_const_style);
    tera.register_filter("has_dyn_style", style_info::has_dyn_style);
}
