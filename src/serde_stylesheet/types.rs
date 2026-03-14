mod lv_align;
mod lv_base_dir;
mod lv_blend_mode;
mod lv_border_side;
mod lv_color;
mod lv_coord;
mod lv_flex_align;
mod lv_flex_flow;
mod lv_grad_dir;
mod lv_grid_align;
mod lv_grid_dsc_array;
mod lv_image_colorkey;
mod lv_layout;
mod lv_opa;
mod lv_part;
mod lv_selector;
mod lv_state;
mod lv_text_align;
mod lv_text_decor;

use serde::Serialize;

pub use lv_align::LVAlign;
pub use lv_base_dir::LVBaseDir;
pub use lv_blend_mode::LVBlendMode;
pub use lv_border_side::LVBorderSide;
pub use lv_color::LVColor;
pub use lv_coord::LVCoord;
pub use lv_coord::LVSize;
pub use lv_flex_align::LVFlexAlign;
pub use lv_flex_flow::LVFlexFlow;
pub use lv_grad_dir::LVGradDir;
pub use lv_grid_align::LVGridAlign;
pub use lv_grid_dsc_array::LVGridDscArray;
pub use lv_image_colorkey::LVImageColorkey;
pub use lv_layout::LVLayout;
pub use lv_opa::LVOpa;
pub use lv_part::LVPart;
pub use lv_selector::LVSelector;
pub use lv_state::LVState;
pub use lv_text_align::LVTextAlign;
pub use lv_text_decor::LVTextDecor;

macro_rules! make_abstract_type {
    (
        $(
            $ty:ty => $variant:ident
        ),*
    ) => {
        #[cfg_attr(test, derive(Debug, PartialEq))]
        #[derive(Serialize)]
        #[serde(untagged)]
        pub enum AbstractType {
            $(
                $variant($ty),
            )*
        }

        $(
            impl From<$ty> for AbstractType {
                fn from(value: $ty) -> Self {
                    AbstractType::$variant(value)
                }
            }
        )*
    };
}

make_abstract_type! {
    i32 => Integer,
    bool => Boolean,
    LVAlign => LVAlign,
    LVBaseDir => LVBaseDir,
    LVBlendMode => LVBlendMode,
    LVBorderSide => LVBorderSide,
    LVColor => LVColor,
    LVCoord => LVCoord,
    LVFlexAlign => LVFlexAlign,
    LVFlexFlow => LVFlexFlow,
    LVGradDir => LVGradDir,
    LVGridAlign => LVGridAlign,
    LVGridDscArray => LVGridDscArray,
    LVImageColorkey => LVImageColorkey,
    LVLayout => LVLayout,
    LVOpa => LVOpa,
    LVSize => LVSize,
    LVTextAlign => LVTextAlign,
    LVTextDecor => LVTextDecor
}
