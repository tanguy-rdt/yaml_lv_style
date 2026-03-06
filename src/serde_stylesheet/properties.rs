use paste::paste;

use serde::{Deserialize, Serialize};

use super::types::AbstractType;
use super::types::LVAlign;
use super::types::LVBaseDir;
use super::types::LVBlendMode;
use super::types::LVBorderSide;
use super::types::LVColor;
use super::types::LVFlexAlign;
use super::types::LVFlexFlow;
use super::types::LVGradDir;
use super::types::LVGridAlign;
use super::types::LVGridDscArray;
use super::types::LVImageColorkey;
use super::types::LVLayout;
use super::types::LVOpa;
use super::types::LVTextAlign;
use super::types::LVTextDecor;

macro_rules! make_properties {
    (
        $(
            { $field:ident, $type:ty }
        )*
    ) => {
        #[cfg_attr(test, derive(Default, Debug, PartialEq))]
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct Properties {
            $(
                $field: Option<$type>,
            )*
        }

        #[allow(dead_code)]
        impl Properties {
            paste! {
                $(
                    pub fn [<get_ $field _ref>](&self) -> Option<&$type> {
                        self.$field.as_ref()
                    }

                    pub fn [<get_ $field _mut>](&mut self) -> Option<&mut $type> {
                        self.$field.as_mut()
                    }
                )*
            }
        }

        #[derive(Serialize)]
        pub struct Property(String, AbstractType);

        impl Property {
            pub fn new(name: String, value: AbstractType) -> Self {
                Property(name, value)
            }

            pub fn get_name(&self) -> &str {
                &self.0
            }

            pub fn get_value_ref(&self) -> &AbstractType {
                &self.1
            }

            pub fn get_value_mut(&mut self) -> &mut AbstractType {
                &mut self.1
            }
        }

        pub type PropertiesMap = Vec<Property>;

        impl From<Properties> for PropertiesMap {
            fn from(props: Properties) -> Self {
                let mut map = Vec::new();

                $(
                    if let Some(value) = props.$field {
                        map.push(Property::new(stringify!($field).to_string(), AbstractType::from(value)));
                    }
                )*

                map
            }
        }
    };
}

make_properties! (
    { width, i32 }
    { min_width, i32 }
    { max_width, i32 }
    { height, i32 }
    { min_height, i32 }
    { max_height, i32 }
    { length, i32 }
    { x, i32 }
    { y, i32 }
    { align, LVAlign }
    { transform_width, i32 }
    { transform_height, i32 }
    { translate_x, i32 }
    { translate_y, i32 }
    { translate_radial, i32 }
    { transform_scale_x, i32 }
    { transform_scale_y, i32 }
    { transform_rotation, i32 }
    { transform_pivot_x, i32 }
    { transform_pivot_y, i32 }
    { transform_skew_x, i32 }
    { transform_skew_y, i32 }
    { pad_top, i32 }
    { pad_bottom, i32 }
    { pad_left, i32 }
    { pad_right, i32 }
    { pad_row, i32 }
    { pad_column, i32 }
    { pad_radial, i32 }
    { margin_top, i32 }
    { margin_bottom, i32 }
    { margin_left, i32 }
    { margin_right, i32 }
    { bg_color, LVColor}
    { bg_opa, LVOpa}
    { bg_grad_color, LVColor}
    { bg_grad_dir, LVGradDir}
    { bg_main_stop, i32}
    { bg_grad_stop, i32}
    { bg_main_opa, LVOpa}
    { bg_grad_opa, LVOpa}
    // { bg_grad, not_yet_implemented }
    // { bg_image_src, not_yet_implemented }
    { bg_image_opa, LVOpa}
    { bg_image_recolor, LVColor}
    { bg_image_recolor_opa, LVOpa}
    { bg_image_tiled, bool}
    { border_color, LVColor}
    { border_opa, LVOpa}
    { border_width, i32}
    { border_side, LVBorderSide}
    { border_post, bool}
    { outline_width, i32}
    { outline_color, LVColor}
    { outline_opa, LVOpa}
    { outline_pad, i32}
    { shadow_width, i32}
    { shadow_offset_x, i32}
    { shadow_offset_y, i32}
    { shadow_spread, i32}
    { shadow_color, LVColor}
    { shadow_opa, LVOpa}
    { image_opa, LVOpa}
    { image_recolor, LVColor}
    { image_recolor_opa, LVOpa}
    { image_colorkey, LVImageColorkey}
    { line_width, i32}
    { line_dash_width, i32}
    { line_dash_gap, i32}
    { line_rounded, bool}
    { line_color, LVColor}
    { line_opa, LVOpa}
    { arc_width, i32}
    { arc_rounded, bool}
    { arc_color, LVColor}
    { arc_opa, LVOpa}
    // { arc_image_src, not_yet_implemented }
    { text_color, LVColor}
    { text_opa, LVOpa}
    // { text_font, not_yet_implemented }
    { text_letter_space, i32}
    { text_line_space, i32}
    { text_decor, LVTextDecor}
    { text_align, LVTextAlign}
    { text_outline_stroke_color, LVColor}
    { text_outline_stroke_width, i32}
    { text_outline_stroke_opa, LVOpa}
    { radius, i32}
    { radial_offset, i32}
    { clip_corner, bool}
    { opa, LVOpa}
    { opa_layered, LVOpa}
    // { color_filter_dsc, not_yet_implemented }
    { color_filter_opa, LVOpa}
    { recolor, LVColor}
    { recolor_opa, LVOpa}
    // { anim, not_yet_implemented }
    // { anim_duration, not_yet_implemented }
    // { transition, not_yet_implemented }
    { blend_mode, LVBlendMode}
    { layout, LVLayout}
    { base_dir, LVBaseDir}
    // { bitmap_mask_src, not_yet_implemented }
    { rotary_sensitivity, i32}
    { flex_flow, LVFlexFlow}
    { flex_main_place, LVFlexAlign}
    { flex_cross_place, LVFlexAlign}
    { flex_track_place, LVFlexAlign}
    { flex_grow, i32}
    { grid_column_dsc_array, LVGridDscArray}
    { grid_column_align, LVGridAlign}
    { grid_row_dsc_array, LVGridDscArray}
    { grid_row_align, LVGridAlign}
    { grid_cell_column_pos, i32}
    { grid_cell_x_align, LVGridAlign}
    { grid_cell_column_span, i32}
    { grid_cell_row_pos, i32}
    { grid_cell_y_align, LVGridAlign}
    { grid_cell_row_span, i32}
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_properties_deserialization() {
        let yaml = r#"
            width: 100
            align: center
            grid_column_dsc_array: [px(100), fr(1)]
            text_color: hex(#F1F5F9)
            "#;

        let properties: Properties = yaml_serde::from_str(yaml).unwrap();

        assert_eq!(properties.width, Some(100));
        assert!(properties.get_align_ref().is_some());
        assert!(properties.get_grid_column_dsc_array_ref().is_some());
        assert!(properties.get_text_color_ref().is_some());
        assert!(properties.get_grid_cell_row_span_ref().is_none());
    }

    #[test]
    fn test_properties_deserialization_with_invalid_field() {
        let yaml = r#"
            largeur: 100
            "#;

        let result: Result<Properties, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_properties_deserialization_with_duplicate_field() {
        let yaml = r#"
            width: 100
            width: 100
            "#;

        let result: Result<Properties, _> = yaml_serde::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_properties_map_from_properties() {
        let yaml = r#"
            width: 100
            align: center
            grid_column_dsc_array: [px(100), fr(1)]
            text_color: hex(#F1F5F9)
            "#;

        let properties: Properties = yaml_serde::from_str(yaml).unwrap();
        let properties_map: PropertiesMap = properties.into();

        assert_eq!(properties_map.len(), 4);

        let width_property = properties_map
            .iter()
            .find(|p| p.get_name() == "width")
            .unwrap();

        assert_eq!(width_property.get_value_ref(), &AbstractType::Integer(100));
        assert!(properties_map.iter().any(|p| p.get_name() == "align"));
        assert!(
            properties_map
                .iter()
                .any(|p| p.get_name() == "grid_column_dsc_array")
        );
        assert!(properties_map.iter().any(|p| p.get_name() == "text_color"));
        assert!(properties_map.iter().any(|p| p.get_name() == "text_color"));
    }
}
