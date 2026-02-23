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

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug, Default)]
#[derive(Deserialize, Serialize)]
pub struct LVProperties {
    pub width: Option<i32>,
    pub min_width: Option<i32>,
    pub max_width: Option<i32>,
    pub height: Option<i32>,
    pub min_height: Option<i32>,
    pub max_height: Option<i32>,
    pub length: Option<i32>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub align: Option<LVAlign>,
    pub transform_width: Option<i32>,
    pub transform_height: Option<i32>,
    pub translate_x: Option<i32>,
    pub translate_y: Option<i32>,
    pub translate_radial: Option<i32>,
    pub transform_scale_x: Option<i32>,
    pub transform_scale_y: Option<i32>,
    pub transform_rotation: Option<i32>,
    pub transform_pivot_x: Option<i32>,
    pub transform_pivot_y: Option<i32>,
    pub transform_skew_x: Option<i32>,
    pub transform_skew_y: Option<i32>,
    pub pad_top: Option<i32>,
    pub pad_bottom: Option<i32>,
    pub pad_left: Option<i32>,
    pub pad_right: Option<i32>,
    pub pad_row: Option<i32>,
    pub pad_column: Option<i32>,
    pub pad_radial: Option<i32>,
    pub margin_top: Option<i32>,
    pub margin_bottom: Option<i32>,
    pub margin_left: Option<i32>,
    pub margin_right: Option<i32>,
    pub bg_color: Option<LVColor>,
    pub bg_opa: Option<LVOpa>,
    pub bg_grad_color: Option<LVColor>,
    pub bg_grad_dir: Option<LVGradDir>,
    pub bg_main_stop: Option<i32>,
    pub bg_grad_stop: Option<i32>,
    pub bg_main_opa: Option<LVOpa>,
    pub bg_grad_opa: Option<LVOpa>,
    // bg_grad: Option<>,
    // bg_image_src: Option<>,
    pub bg_image_opa: Option<LVOpa>,
    pub bg_image_recolor: Option<LVColor>,
    pub bg_image_recolor_opa: Option<LVOpa>,
    pub bg_image_tiled: Option<bool>,
    pub border_color: Option<LVColor>,
    pub border_opa: Option<LVOpa>,
    pub border_width: Option<i32>,
    pub border_side: Option<LVBorderSide>,
    pub border_post: Option<bool>,
    pub outline_width: Option<i32>,
    pub outline_color: Option<LVColor>,
    pub outline_opa: Option<LVOpa>,
    pub outline_pad: Option<i32>,
    pub shadow_width: Option<i32>,
    pub shadow_offset_x: Option<i32>,
    pub shadow_offset_y: Option<i32>,
    pub shadow_spread: Option<i32>,
    pub shadow_color: Option<LVColor>,
    pub shadow_opa: Option<LVOpa>,
    pub image_opa: Option<LVOpa>,
    pub image_recolor: Option<LVColor>,
    pub image_recolor_opa: Option<LVOpa>,
    // image_colorkey: Option<>,
    pub line_width: Option<i32>,
    pub line_dash_width: Option<i32>,
    pub line_dash_gap: Option<i32>,
    pub line_rounded: Option<bool>,
    pub line_color: Option<LVColor>,
    pub line_opa: Option<LVOpa>,
    pub arc_width: Option<i32>,
    pub arc_rounded: Option<bool>,
    pub arc_color: Option<LVColor>,
    pub arc_opa: Option<LVOpa>,
    // arc_image_src: Option<>,
    pub text_color: Option<LVColor>,
    pub text_opa: Option<LVOpa>,
    // text_font: Option<>,
    pub text_letter_space: Option<i32>,
    pub text_line_space: Option<i32>,
    // text_decor: Option<>,
    pub text_align: Option<LVTextAlign>,
    pub text_outline_stroke_color: Option<LVColor>,
    pub text_outline_stroke_width: Option<i32>,
    pub text_outline_stroke_opa: Option<LVOpa>,
    pub radius: Option<i32>,
    pub radial_offset: Option<i32>,
    pub clip_corner: Option<bool>,
    pub opa: Option<LVOpa>,
    pub opa_layered: Option<LVOpa>,
    // color_filter_dsc: Option<>,
    pub color_filter_opa: Option<LVOpa>,
    pub recolor: Option<LVColor>,
    pub recolor_opa: Option<LVOpa>,
    // anim: Option<>,
    // anim_duration: Option<>,
    // transition: Option<>,
    // blend_mode: Option<>,
    // layout: Option<>,
    // base_dir: Option<>,
    // bitmap_mask_src: Option<>,
    // rotary_sensitivity: Option<>,
    pub flex_flow: Option<LVFlexFlow>,
    pub flex_main_place: Option<LVFlexAlign>,
    pub flex_cross_place: Option<LVFlexAlign>,
    pub flex_track_place: Option<LVFlexAlign>,
    pub flex_grow: Option<i32>,
    // grid_column_dsc_array: Option<>,
    pub grid_column_align: Option<LVGridAlign>,
    // grid_row_dsc_array: Option<>,
    pub grid_row_align: Option<LVGridAlign>,
    pub grid_cell_column_pos: Option<i32>,
    pub grid_cell_x_align: Option<LVGridAlign>,
    pub grid_cell_column_span: Option<i32>,
    pub grid_cell_row_pos: Option<i32>,
    pub grid_cell_y_align: Option<LVGridAlign>,
    pub grid_cell_row_span: Option<i32>,
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
        assert_eq!(parsed.bg_grad_color, Some(LVColor::Rgb(255, 255, 255)));
        assert_eq!(props.bg_grad_dir, parsed.bg_grad_dir);
        assert_eq!(props.border_side, parsed.border_side);
        assert_eq!(props.text_align, parsed.text_align);
        assert_eq!(props.flex_flow, parsed.flex_flow);
        assert_eq!(props.flex_main_place, parsed.flex_main_place);
        assert_eq!(props.grid_column_align, parsed.grid_column_align);
    }
}