use serde::{Deserialize, Serialize};

use super::lv_types::lv_align::LVAlign;
use super::lv_types::lv_border_side::LVBorderSide;
use super::lv_types::lv_color::LVColor;
use super::lv_types::lv_flex_align::LVFlexAlign;
use super::lv_types::lv_flex_flow::LVFlexFlow;
use super::lv_types::lv_grad_dir::LVGradDir;
use super::lv_types::lv_grid_align::LVGridAlign;
use super::lv_types::lv_opa::LVOpa;
use super::lv_types::lv_text_align::LVTextAlign;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct LVProperties {
    width: Option<i32>,
    min_width: Option<i32>,
    max_width: Option<i32>,
    height: Option<i32>,
    min_height: Option<i32>,
    max_height: Option<i32>,
    length: Option<i32>,
    x: Option<i32>,
    y: Option<i32>,
    align: Option<LVAlign>,
    transform_width: Option<i32>,
    transform_height: Option<i32>,
    translate_x: Option<i32>,
    translate_y: Option<i32>,
    translate_radial: Option<i32>,
    transform_scale_x: Option<i32>,
    transform_scale_y: Option<i32>,
    transform_rotation: Option<i32>,
    transform_pivot_x: Option<i32>,
    transform_pivot_y: Option<i32>,
    transform_skew_x: Option<i32>,
    transform_skew_y: Option<i32>,
    pad_top: Option<i32>,
    pad_bottom: Option<i32>,
    pad_left: Option<i32>,
    pad_right: Option<i32>,
    pad_row: Option<i32>,
    pad_column: Option<i32>,
    pad_radial: Option<i32>,
    margin_top: Option<i32>,
    margin_bottom: Option<i32>,
    margin_left: Option<i32>,
    margin_right: Option<i32>,
    bg_color: Option<LVColor>,
    bg_opa: Option<LVOpa>,
    bg_grad_color: Option<LVColor>,
    bg_grad_dir: Option<LVGradDir>,
    bg_main_stop: Option<i32>,
    bg_grad_stop: Option<i32>,
    bg_main_opa: Option<LVOpa>,
    bg_grad_opa: Option<LVOpa>,
    // bg_grad: Option<>,
    // bg_image_src: Option<>,
    bg_image_opa: Option<LVOpa>,
    bg_image_recolor: Option<LVColor>,
    bg_image_recolor_opa: Option<LVOpa>,
    bg_image_tiled: Option<bool>,
    border_color: Option<LVColor>,
    border_opa: Option<LVOpa>,
    border_width: Option<i32>,
    border_side: Option<LVBorderSide>,
    border_post: Option<bool>,
    outline_width: Option<i32>,
    outline_color: Option<LVColor>,
    outline_opa: Option<LVOpa>,
    outline_pad: Option<i32>,
    shadow_width: Option<i32>,
    shadow_offset_x: Option<i32>,
    shadow_offset_y: Option<i32>,
    shadow_spread: Option<i32>,
    shadow_color: Option<LVColor>,
    shadow_opa: Option<LVOpa>,
    image_opa: Option<LVOpa>,
    image_recolor: Option<LVColor>,
    image_recolor_opa: Option<LVOpa>,
    // image_colorkey: Option<>,
    line_width: Option<i32>,
    line_dash_width: Option<i32>,
    line_dash_gap: Option<i32>,
    line_rounded: Option<bool>,
    line_color: Option<LVColor>,
    line_opa: Option<LVOpa>,
    arc_width: Option<i32>,
    arc_rounded: Option<bool>,
    arc_color: Option<LVColor>,
    arc_opa: Option<LVOpa>,
    // arc_image_src: Option<>,
    text_color: Option<LVColor>,
    text_opa: Option<LVOpa>,
    // text_font: Option<>,
    text_letter_space: Option<i32>,
    text_line_space: Option<i32>,
    // text_decor: Option<>,
    text_align: Option<LVTextAlign>,
    text_outline_stroke_color: Option<LVColor>,
    text_outline_stroke_width: Option<i32>,
    text_outline_stroke_opa: Option<LVOpa>,
    radius: Option<i32>,
    radial_offset: Option<i32>,
    clip_corner: Option<bool>,
    opa: Option<LVOpa>,
    opa_layered: Option<LVOpa>,
    // color_filter_dsc: Option<>,
    color_filter_opa: Option<LVOpa>,
    recolor: Option<LVColor>,
    recolor_opa: Option<LVOpa>,
    // anim: Option<>,
    // anim_duration: Option<>,
    // transition: Option<>,
    // blend_mode: Option<>,
    // layout: Option<>,
    // base_dir: Option<>,
    // bitmap_mask_src: Option<>,
    // rotary_sensitivity: Option<>,
    flex_flow: Option<LVFlexFlow>,
    flex_main_place: Option<LVFlexAlign>,
    flex_cross_place: Option<LVFlexAlign>,
    flex_track_place: Option<LVFlexAlign>,
    flex_grow: Option<i32>,
    // grid_column_dsc_array: Option<>,
    grid_column_align: Option<LVGridAlign>,
    // grid_row_dsc_array: Option<>,
    grid_row_align: Option<LVGridAlign>,
    grid_cell_column_pos: Option<i32>,
    grid_cell_x_align: Option<LVGridAlign>,
    grid_cell_column_span: Option<i32>,
    grid_cell_row_pos: Option<i32>,
    grid_cell_y_align: Option<LVGridAlign>,
    grid_cell_row_span: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml_serde;

    #[test]
    fn test_lv_properties_serde() {
        let props = LVProperties {
            width: Some(100),
            align: Some(LVAlign::LvAlignDefault),
            bg_color: Some(LVColor::Rgb(255, 255, 255)),
            bg_opa: Some(LVOpa::LVOpa0),
            bg_grad_color: Some(LVColor::Hex(0xffffff)),
            bg_grad_dir: Some(LVGradDir::LVGradDirConical),
            border_side: Some(LVBorderSide::LVBorderSideBottom),
            text_align: Some(LVTextAlign::LvTextAlignAuto),
            flex_flow: Some(LVFlexFlow::LVFlexFlowColumn),
            flex_main_place: Some(LVFlexAlign::LVFlexAlignEnd),
            grid_column_align: Some(LVGridAlign::LVGridAlignCenter),
            ..Default::default()
        };

        let yaml = yaml_serde::to_string(&props).unwrap();
        let parsed: LVProperties = yaml_serde::from_str(&yaml).unwrap();

        assert_eq!(props.width, parsed.width);
        assert_eq!(props.align, parsed.align);
        assert_eq!(props.bg_color, parsed.bg_color);
        assert_eq!(props.bg_opa, parsed.bg_opa);
        assert_eq!(props.bg_grad_color, parsed.bg_grad_color);
        assert_eq!(props.bg_grad_dir, parsed.bg_grad_dir);
        assert_eq!(props.border_side, parsed.border_side);
        assert_eq!(props.text_align, parsed.text_align);
        assert_eq!(props.flex_flow, parsed.flex_flow);
        assert_eq!(props.flex_main_place, parsed.flex_main_place);
        assert_eq!(props.grid_column_align, parsed.grid_column_align);
    }
}