# Style Properties

> [!NOTE]
> This documentation is inspired by the official [LVGL documentation: Style Properties](https://docs.lvgl.io/master/common-widget-features/styles/style-properties.html#).

## Table of Contents

- [Size And Position](#size-and-position)
  - [`width`](#width) — Sets width of Widget
  - [`min_width`](#min_width) — Sets a minimal width
  - [`max_width`](#max_width) — Sets a maximal width
  - [`height`](#height) — Sets height of Widget
  - [`min_height`](#min_height) — Sets a minimal height
  - [`max_height`](#max_height) — Sets a maximal height
  - [`x`](#x) — Set X coordinate of Widget considering the align setting
  - [`y`](#y) — Set Y coordinate of Widget considering the align setting
  - [`align`](#align) — Set the alignment which tells from which point of the parent the X and Y coordinates should be interpreted
  - [`length`](#length) — Its meaning depends on the type of Widget
  - [`transform_width`](#transform_width) — Make Widget wider on both sides with this value
  - [`transform_height`](#transform_height) — Make Widget higher on both sides with this value
  - [`transform_scale_x`](#transform_scale_x) — Zoom Widget horizontally
  - [`transform_scale_y`](#transform_scale_y) — Zoom Widget vertically
  - [`transform_rotation`](#transform_rotation) — Rotate Widget
  - [`transform_pivot_x`](#transform_pivot_x) — Set pivot point's X coordinate for transformations
  - [`transform_pivot_y`](#transform_pivot_y) — Set pivot point's Y coordinate for transformations
  - [`transform_skew_x`](#transform_skew_x) — Skew Widget horizontally
  - [`transform_skew_y`](#transform_skew_y) — Skew Widget vertically
  - [`translate_x`](#translate_x) — Move Widget with this value in X direction
  - [`translate_y`](#translate_y) — Move Widget with this value in Y direction
  - [`translate_radial`](#translate_radial) — Move object around the centre of the parent object, e
  - [`radial_offset`](#radial_offset) — Move start point of object (e
- [Padding](#padding)
  - [`pad_top`](#pad_top) — Sets the padding on the top
  - [`pad_bottom`](#pad_bottom) — Sets the padding on the bottom
  - [`pad_left`](#pad_left) — Sets the padding on the left
  - [`pad_right`](#pad_right) — Sets the padding on the right
  - [`pad_row`](#pad_row) — Sets the padding between the rows
  - [`pad_column`](#pad_column) — Sets the padding between the columns
  - [`pad_radial`](#pad_radial) — Pad text labels away from the scale ticks or remainder of the LV_PART_
- [Margin](#margin)
  - [`margin_top`](#margin_top) — Sets margin on the top
  - [`margin_bottom`](#margin_bottom) — Sets margin on the bottom
  - [`margin_left`](#margin_left) — Sets margin on the left
  - [`margin_right`](#margin_right) — Sets margin on the right
- [Background](#background)
  - [`bg_color`](#bg_color) — Set background color of Widget
  - [`bg_opa`](#bg_opa) — Set opacity of the background
  - [`bg_grad_color`](#bg_grad_color) — Set gradient color of the background
  - [`bg_grad_dir`](#bg_grad_dir) — Set direction of the gradient of the background
  - [`bg_main_stop`](#bg_main_stop) — Set point from which background color should start for gradients
  - [`bg_grad_stop`](#bg_grad_stop) — Set point from which background's gradient color should start
  - [`bg_main_opa`](#bg_main_opa) — Set opacity of the first gradient color
  - [`bg_grad_opa`](#bg_grad_opa) — Set opacity of the second gradient color
  - :heavy_exclamation_mark: [`bg_grad`](#bg_grad) — Not yet implemented
  - :heavy_exclamation_mark: [`bg_image_src`](#bg_image_src) — Not yet implemented
  - [`bg_image_opa`](#bg_image_opa) — Set opacity of the background image
  - [`bg_image_recolor`](#bg_image_recolor) — Set a color to mix to the background image
  - [`bg_image_recolor_opa`](#bg_image_recolor_opa) — Set intensity of background image recoloring
  - [`bg_image_tiled`](#bg_image_tiled) — If enabled the background image will be tiled
- [Border](#border)
  - [`border_color`](#border_color) — Set color of the border
  - [`border_opa`](#border_opa) — Set opacity of the border
  - [`border_width`](#border_width) — Set width of the border
  - [`border_side`](#border_side) — Set which side(s) the border should be drawn on
  - [`border_post`](#border_post) — Sets whether the border should be drawn before or after the children are drawn
- [Outline](#outline)
  - [`outline_width`](#outline_width) — Set width of outline in pixels
  - [`outline_color`](#outline_color) — Set color of outline
  - [`outline_opa`](#outline_opa) — Set opacity of outline
  - [`outline_pad`](#outline_pad) — Set padding of outline, i
- [Shadow](#shadow)
  - [`shadow_width`](#shadow_width) — Set width of the shadow
  - [`shadow_offset_x`](#shadow_offset_x) — Set an offset on the shadow in X direction
  - [`shadow_offset_y`](#shadow_offset_y) — Set an offset on the shadow in Y direction
  - [`shadow_spread`](#shadow_spread) — Make shadow calculation use a larger or smaller rectangle as base
  - [`shadow_color`](#shadow_color) — Set color of shadow
  - [`shadow_opa`](#shadow_opa) — Set opacity of shadow
- [Image](#image)
  - [`image_opa`](#image_opa) — Set opacity of an image
  - [`image_recolor`](#image_recolor) — Set color to mix with the image
  - [`image_recolor_opa`](#image_recolor_opa) — Set intensity of color mixing
  - [`image_colorkey`](#image_colorkey) — Set image colorkey definition
- [Line](#line)
  - [`line_width`](#line_width) — Set width of lines in pixels
  - [`line_dash_width`](#line_dash_width) — Set width of dashes in pixels
  - [`line_dash_gap`](#line_dash_gap) — Set gap between dashes in pixels
  - [`line_rounded`](#line_rounded) — Make end points of the lines rounded
  - [`line_color`](#line_color) — Set color of lines
  - [`line_opa`](#line_opa) — Set opacity of lines
- [Arc](#arc)
  - [`arc_width`](#arc_width) — Set width (thickness) of arcs in pixels
  - [`arc_rounded`](#arc_rounded) — Make end points of arcs rounded
  - [`arc_color`](#arc_color) — Set color of arc
  - [`arc_opa`](#arc_opa) — Set opacity of arcs
  - :heavy_exclamation_mark: [`arc_image_src`](#arc_image_src) — Not yet implemented
- [Text](#text)
  - [`text_color`](#text_color) — Sets color of text
  - [`text_opa`](#text_opa) — Set opacity of text
  - :heavy_exclamation_mark: [`text_font`](#text_font) — Not yet implemented
  - [`text_letter_space`](#text_letter_space) — Set letter space in pixels
  - [`text_line_space`](#text_line_space) — Set line space in pixels
  - [`text_decor`](#text_decor) — Set decoration for the text
  - [`text_align`](#text_align) — Set how to align the lines of the text
  - [`text_outline_stroke_color`](#text_outline_stroke_color) — Sets the color of letter outline stroke
  - [`text_outline_stroke_width`](#text_outline_stroke_width) — Set the letter outline stroke width in pixels
  - [`text_outline_stroke_opa`](#text_outline_stroke_opa) — Set the opacity of the letter outline stroke
- [Blur](#blur)
  - [`blur_radius`](#blur_radius) — Sets the intensity of blurring
  - [`blur_backdrop`](#blur_backdrop) — If true the background of the widget will be blurred
  - [`blur_quality`](#blur_quality) — Setting to LV_BLUR_QUALITY_SPEED the blurring algorithm will prefer speed over quality
- [Drop Shadow](#drop-shadow)
  - [`drop_shadow_radius`](#drop_shadow_radius) — Sets the intensity of blurring
  - [`drop_shadow_offset_x`](#drop_shadow_offset_x) — Set an offset on the shadow in pixels in X direction
  - [`drop_shadow_offset_y`](#drop_shadow_offset_y) — Set an offset on the shadow in pixels in Y direction
  - [`drop_shadow_color`](#drop_shadow_color) — Set the color of the shadow
  - [`drop_shadow_quality`](#drop_shadow_quality) — Setting to LV_BLUR_QUALITY_SPEED the blurring algorithm will prefer speed over quality
- [Miscellaneous](#miscellaneous)
  - [`radius`](#radius) — Set radius on every corner
  - [`clip_corner`](#clip_corner) — Enable clipping of content that overflows rounded corners of parent Widget
  - [`opa`](#opa) — Scale down all opacity values of the Widget by this factor
  - [`opa_layered`](#opa_layered) — First draw Widget on a layer, then scale down the layer opacity by this factor
  - :heavy_exclamation_mark: [`color_filter_dsc`](#color_filter_dsc) — Not yet implemented
  - [`color_filter_opa`](#color_filter_opa) — Set the intensity of mixing of color filter
  - [`recolor`](#recolor) — Set a color to mix to the Widget
  - [`recolor_opa`](#recolor_opa) — Set the intensity of color mixing
  - :heavy_exclamation_mark: [`anim`](#anim) — Not yet implemented
  - :heavy_exclamation_mark: [`anim_duration`](#anim_duration) — Not yet implemented
  - :heavy_exclamation_mark: [`transition`](#transition) — Not yet implemented
  - [`blend_mode`](#blend_mode) — Describes how to blend the colors to the background
  - [`layout`](#layout) — Set layout of Widget
  - [`base_dir`](#base_dir) — Set base direction of Widget
  - :heavy_exclamation_mark: [`bitmap_mask_src`](#bitmap_mask_src) — Not yet implemented
  - [`rotary_sensitivity`](#rotary_sensitivity) — Adjust sensitivity for rotary encoders
- [Flex](#flex)
  - [`flex_flow`](#flex_flow) — Defines in which direction the flex layout should arrange the children
  - [`flex_main_place`](#flex_main_place) — Defines how to align the children in the direction of flex flow
  - [`flex_cross_place`](#flex_cross_place) — Defines how to align the children perpendicular to the direction of flex flow
  - [`flex_track_place`](#flex_track_place) — Defines how to align the tracks of the flow
  - [`flex_grow`](#flex_grow) — Defines how much space to take proportionally from the free space of the Widget's track
- [Grid](#grid)
  - [`grid_column_dsc_array`](#grid_column_dsc_array) — An array to describe the columns of the grid
  - [`grid_column_align`](#grid_column_align) — Defines how to distribute the columns
  - [`grid_row_dsc_array`](#grid_row_dsc_array) — An array to describe the rows of the grid
  - [`grid_row_align`](#grid_row_align) — Defines how to distribute the rows
  - [`grid_cell_column_pos`](#grid_cell_column_pos) — Set column in which Widget should be placed
  - [`grid_cell_x_align`](#grid_cell_x_align) — Set how to align Widget horizontally within its grid cell
  - [`grid_cell_column_span`](#grid_cell_column_span) — Set how many columns Widget should span
  - [`grid_cell_row_pos`](#grid_cell_row_pos) — Set row in which Widget should be placed
  - [`grid_cell_y_align`](#grid_cell_y_align) — Set how to align Widget vertically within its grid cell
  - [`grid_cell_row_span`](#grid_cell_row_span) — Set how many rows Widget should span

## Size And Position

Properties related to size, position, alignment and layout of Widgets.

### `width`

Sets width of Widget.

**Allowed values:** Pixel, percentage or LV_SIZE_CONTENT (alias: content) values (e.g. '10', '10%', 'content').

> [!NOTE]
> Percentage values are relative to the width of the parent's content area.

**Default:** `Widget dependent` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `min_width`

Sets a minimal width.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to the width of the parent's content area.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `max_width`

Sets a maximal width.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to the width of the parent's content area.

**Default:** `LV_COORD_MAX` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `height`

Sets height of Widget.

**Allowed values:** Pixel, percentage or LV_SIZE_CONTENT (alias: content) values (e.g. '10', '10%', 'content').

> [!NOTE]
> Percentage values are relative to the height of the parent's content area.

**Default:** `Widget dependent` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `min_height`

Sets a minimal height.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to the height of the parent's content area.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `max_height`

Sets a maximal height.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to the height of the parent's content area.

**Default:** `LV_COORD_MAX` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `x`

Set X coordinate of Widget considering the align setting.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to the width of the parent's content area.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `y`

Set Y coordinate of Widget considering the align setting.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to the height of the parent's content area.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `align`

Set the alignment which tells from which point of the parent the X and Y coordinates should be interpreted.

**Allowed values:**
- `LV_ALIGN_DEFAULT` *(alias: `default`)*
- `LV_ALIGN_TOP_LEFT` *(alias: `top_left`)*
- `LV_ALIGN_TOP_MID` *(alias: `top_mid`)*
- `LV_ALIGN_TOP_RIGHT` *(alias: `top_right`)*
- `LV_ALIGN_BOTTOM_LEFT` *(alias: `bottom_left`)*
- `LV_ALIGN_BOTTOM_MID` *(alias: `bottom_mid`)*
- `LV_ALIGN_BOTTOM_RIGHT` *(alias: `bottom_right`)*
- `LV_ALIGN_LEFT_MID` *(alias: `left_mid`)*
- `LV_ALIGN_RIGHT_MID` *(alias: `right_mid`)*
- `LV_ALIGN_CENTER` *(alias: `center`)*
- `LV_ALIGN_OUT_TOP_LEFT` *(alias: `out_top_left`)*
- `LV_ALIGN_OUT_TOP_MID` *(alias: `out_top_mid`)*
- `LV_ALIGN_OUT_TOP_RIGHT` *(alias: `out_top_right`)*
- `LV_ALIGN_OUT_BOTTOM_LEFT` *(alias: `out_bottom_left`)*
- `LV_ALIGN_OUT_BOTTOM_MID` *(alias: `out_bottom_mid`)*
- `LV_ALIGN_OUT_BOTTOM_RIGHT` *(alias: `out_bottom_right`)*
- `LV_ALIGN_OUT_LEFT_TOP` *(alias: `out_left_top`)*
- `LV_ALIGN_OUT_LEFT_MID` *(alias: `out_left_mid`)*
- `LV_ALIGN_OUT_LEFT_BOTTOM` *(alias: `out_left_bottom`)*
- `LV_ALIGN_OUT_RIGHT_TOP` *(alias: `out_right_top`)*
- `LV_ALIGN_OUT_RIGHT_MID` *(alias: `out_right_mid`)*
- `LV_ALIGN_OUT_RIGHT_BOTTOM` *(alias: `out_right_bottom`)*

**Default:** `LV_ALIGN_DEFAULT` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `length`

Its meaning depends on the type of Widget. For example in case of lv_scale it means the length of the ticks.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `transform_width`

Make Widget wider on both sides with this value.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to Widget's width.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `transform_height`

Make Widget higher on both sides with this value.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to Widget's height.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `transform_scale_x`

Zoom Widget horizontally.

**Allowed values:** Integer values. 256 means normal size, 128 half size, 512 double size, and so on.

**Default:** `256` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `transform_scale_y`

Zoom Widget vertically.

**Allowed values:** Integer values. 256 means normal size, 128 half size, 512 double size, and so on.

**Default:** `256` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `transform_rotation`

Rotate Widget.

**Allowed values:** Integer values in 0.1 degree units (e.g. '450' means 45 deg).

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `transform_pivot_x`

Set pivot point's X coordinate for transformations. Relative to Widget's top left corner.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `transform_pivot_y`

Set pivot point's Y coordinate for transformations. Relative to Widget's top left corner.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `transform_skew_x`

Skew Widget horizontally.

**Allowed values:** Integer values in 0.1 degree units (e.g. '450' means 45 deg).

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `transform_skew_y`

Skew Widget vertically.

**Allowed values:** Integer values in 0.1 degree units (e.g. '450' means 45 deg).

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `translate_x`

Move Widget with this value in X direction. Applied after layouts, aligns and other positioning.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to Widget's width.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `translate_y`

Move Widget with this value in Y direction. Applied after layouts, aligns and other positioning.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

> [!NOTE]
> Percentage values are relative to Widget's height.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `translate_radial`

Move object around the centre of the parent object, e.g. around the circumference of a scale.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `radial_offset`

Move start point of object (e.g. scale tick) radially.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Padding

Properties to describe spacing between the parent's sides and the children and among the children.

### `pad_top`

Sets the padding on the top. It makes the content area smaller in this direction.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `pad_bottom`

Sets the padding on the bottom. It makes the content area smaller in this direction.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `pad_left`

Sets the padding on the left. It makes the content area smaller in this direction.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `pad_right`

Sets the padding on the right. It makes the content area smaller in this direction.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `pad_row`

Sets the padding between the rows. Used by the layouts.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `pad_column`

Sets the padding between the columns. Used by the layouts.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `pad_radial`

Pad text labels away from the scale ticks or remainder of the LV_PART_.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Margin

Properties to describe spacing around a Widget.

### `margin_top`

Sets margin on the top. Widget will keep this space from its siblings in layouts.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `margin_bottom`

Sets margin on the bottom. Widget will keep this space from its siblings in layouts.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `margin_left`

Sets margin on the left. Widget will keep this space from its siblings in layouts.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `margin_right`

Sets margin on the right. Widget will keep this space from its siblings in layouts.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Background

Properties to describe the background color and image of Widget.

### `bg_color`

Set background color of Widget.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0xffffff` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_opa`

Set opacity of the background.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_TRANSP` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_grad_color`

Set gradient color of the background. Used only if bg_grad_dir is not LV_GRAD_DIR_NONE.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_grad_dir`

Set direction of the gradient of the background.

**Allowed values:**
- `LV_GRAD_DIR_NONE` *(alias: `none`)*
- `LV_GRAD_DIR_VER` *(alias: `vertical`)*
- `LV_GRAD_DIR_HOR` *(alias: `horizontal`)*
- `LV_GRAD_DIR_LINEAR` *(alias: `linear`)*
- `LV_GRAD_DIR_RADIAL` *(alias: `radial`)*
- `LV_GRAD_DIR_CONICAL` *(alias: `conical`)*

**Default:** `LV_GRAD_DIR_NONE` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_main_stop`

Set point from which background color should start for gradients.

**Allowed values:** Integer values between 0 and 255. 0 means top/left side, 255 the bottom/right side, 128 the center.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_grad_stop`

Set point from which background's gradient color should start.

**Allowed values:** Integer values between 0 and 255. 0 means top/left side, 255 the bottom/right side, 128 the center.

**Default:** `255` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_main_opa`

Set opacity of the first gradient color.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_grad_opa`

Set opacity of the second gradient color.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_grad`

> [!IMPORTANT]
> Not yet implemented

Set gradient definition. Wraps bg_grad_color, bg_grad_dir, bg_main_stop and bg_grad_stop into one descriptor and allows gradients with more colors. If set, other gradient related properties are ignored.

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_image_src`

> [!IMPORTANT]
> Not yet implemented

Set a background image. Can be a pointer to lv_image_dsc_t, a path to a file or an LV_SYMBOL_...

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `bg_image_opa`

Set opacity of the background image.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_image_recolor`

Set a color to mix to the background image.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_image_recolor_opa`

Set intensity of background image recoloring.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_TRANSP` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bg_image_tiled`

If enabled the background image will be tiled.

**Allowed values:** true, false

**Default:** `false` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Border

Properties to describe the borders.

### `border_color`

Set color of the border.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `border_opa`

Set opacity of the border.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `border_width`

Set width of the border. Only pixel values can be used.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `border_side`

Set which side(s) the border should be drawn on. OR-ed values can be used.

**Allowed values:**
- `LV_BORDER_SIDE_NONE` *(alias: `none`)*
- `LV_BORDER_SIDE_BOTTOM` *(alias: `bottom`)*
- `LV_BORDER_SIDE_TOP` *(alias: `top`)*
- `LV_BORDER_SIDE_LEFT` *(alias: `left`)*
- `LV_BORDER_SIDE_RIGHT` *(alias: `right`)*
- `LV_BORDER_SIDE_FULL` *(alias: `full`)*
- `LV_BORDER_SIDE_INTERNAL` *(alias: `internal`)*

**Default:** `LV_BORDER_SIDE_NONE` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `border_post`

Sets whether the border should be drawn before or after the children are drawn.

**Allowed values:** true (after children), false (before children).

**Default:** `false` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Outline

Properties to describe the outline. It's like a border but drawn outside of the rectangles.

### `outline_width`

Set width of outline in pixels.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `outline_color`

Set color of outline.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `outline_opa`

Set opacity of outline.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `outline_pad`

Set padding of outline, i.e. the gap between Widget and the outline.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

## Shadow

Properties to describe the shadow drawn under the rectangles.

### `shadow_width`

Set width of the shadow. The value should be >= 0.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `shadow_offset_x`

Set an offset on the shadow in X direction.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `shadow_offset_y`

Set an offset on the shadow in Y direction.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `shadow_spread`

Make shadow calculation use a larger or smaller rectangle as base.

**Allowed values:** Pixel values. Positive values make the shadow area larger, negative values smaller.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `shadow_color`

Set color of shadow.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `shadow_opa`

Set opacity of shadow.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

## Image

Properties to describe the images.

### `image_opa`

Set opacity of an image.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `image_recolor`

Set color to mix with the image.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `image_recolor_opa`

Set intensity of color mixing.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_TRANSP` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `image_colorkey`

Set image colorkey definition. Pixels whose color falls between low_color and high_color will be transparent.

**Allowed values:** A color range defined by a low and high LVColor bound.
```yaml
image_colorkey:
  low: "#000000"
  high: "#0F0F0F"
# or inline
image_colorkey: { low: "#000000", high: "#0F0F0F" }
```

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Line

Properties to describe line-like Widgets.

### `line_width`

Set width of lines in pixels.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `line_dash_width`

Set width of dashes in pixels. Dash works only on horizontal and vertical lines.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `line_dash_gap`

Set gap between dashes in pixels. Dash works only on horizontal and vertical lines.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `line_rounded`

Make end points of the lines rounded.

**Allowed values:** true (rounded), false (perpendicular line ending).

**Default:** `false` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `line_color`

Set color of lines.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `line_opa`

Set opacity of lines.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Arc

Properties to describe arc-like Widgets.

### `arc_width`

Set width (thickness) of arcs in pixels.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** Yes

---

### `arc_rounded`

Make end points of arcs rounded.

**Allowed values:** true (rounded), false (perpendicular line ending).

**Default:** `false` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `arc_color`

Set color of arc.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `arc_opa`

Set opacity of arcs.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `arc_image_src`

> [!IMPORTANT]
> Not yet implemented

Set an image from which the arc will be masked out. Useful to display complex effects on arcs.

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Text

Properties to describe the properties of text. All these properties are inherited.

### `text_color`

Sets color of text.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_opa`

Set opacity of text.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_font`

> [!IMPORTANT]
> Not yet implemented

Set font of text.

**Default:** `LV_FONT_DEFAULT` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_letter_space`

Set letter space in pixels.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_line_space`

Set line space in pixels.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_decor`

Set decoration for the text. OR-ed values can be used.

**Allowed values:**
- `LV_TEXT_DECOR_NONE` *(alias: `none`)*
- `LV_TEXT_DECOR_UNDERLINE` *(alias: `underline`)*
- `LV_TEXT_DECOR_STRIKETHROUGH` *(alias: `strikethrough`)*

**Default:** `LV_TEXT_DECOR_NONE` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_align`

Set how to align the lines of the text. Does not align the Widget itself, only the lines inside it.

**Allowed values:**
- `LV_TEXT_ALIGN_AUTO` *(alias: `auto`)*
- `LV_TEXT_ALIGN_LEFT` *(alias: `left`)*
- `LV_TEXT_ALIGN_CENTER` *(alias: `center`)*
- `LV_TEXT_ALIGN_RIGHT` *(alias: `right`)*

**Default:** `LV_TEXT_ALIGN_AUTO` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_outline_stroke_color`

Sets the color of letter outline stroke.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_outline_stroke_width`

Set the letter outline stroke width in pixels.

**Allowed values:** Pixel values (e.g. '10').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `text_outline_stroke_opa`

Set the opacity of the letter outline stroke.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Blur

Blur the widget or its background

### `blur_radius`

Sets the intensity of blurring. Applied on each lv_part separately before the children are rendered.

**Allowed values:** Integer

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `blur_backdrop`

If true the background of the widget will be blurred. The part should have < 100% opacity to make it visible. If false the given part will be blurred when it's rendered but before drawing the children.

**Allowed values:** Boolean

**Default:** `false` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `blur_quality`

Setting to LV_BLUR_QUALITY_SPEED the blurring algorithm will prefer speed over quality. LV_BLUR_QUALITY_PRECISION will force using higher quality but slower blur. With LV_BLUR_QUALITY_AUTO the quality will be selected automatically.

**Allowed values:**
- `LV_BLUR_QUALITY_AUTO` *(alias: `auto`)*
- `LV_BLUR_QUALITY_SPEED` *(alias: `speed`)*
- `LV_BLUR_QUALITY_PRECISION` *(alias: `precision`)*

**Default:** `LV_BLUR_QUALITY_AUTO` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Drop Shadow

Take an A8 snapshot of the given part and blur it.

### `drop_shadow_radius`

Sets the intensity of blurring. Applied on each lv_part separately before the children are rendered.

**Allowed values:** Integer

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `drop_shadow_offset_x`

Set an offset on the shadow in pixels in X direction.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `drop_shadow_offset_y`

Set an offset on the shadow in pixels in Y direction.

**Allowed values:** Pixel or percentage values (e.g. '10', '10%').

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `drop_shadow_color`

Set the color of the shadow.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `drop_shadow_quality`

Setting to LV_BLUR_QUALITY_SPEED the blurring algorithm will prefer speed over quality. LV_BLUR_QUALITY_PRECISION will force using higher quality but slower blur. With LV_BLUR_QUALITY_AUTO the quality will be selected automatically.

**Allowed values:**
- `LV_BLUR_QUALITY_AUTO` *(alias: `auto`)*
- `LV_BLUR_QUALITY_SPEED` *(alias: `speed`)*
- `LV_BLUR_QUALITY_PRECISION` *(alias: `precision`)*

**Default:** `LV_BLUR_QUALITY_PRECISION` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Miscellaneous

Mixed properties for various purposes.

### `radius`

Set radius on every corner.

**Allowed values:** Pixel values (>= 0) or LV_RADIUS_CIRCLE for max. radius.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `clip_corner`

Enable clipping of content that overflows rounded corners of parent Widget.

**Allowed values:** true, false

**Default:** `false` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `opa`

Scale down all opacity values of the Widget by this factor.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `opa_layered`

First draw Widget on a layer, then scale down the layer opacity by this factor.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_COVER` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `color_filter_dsc`

> [!IMPORTANT]
> Not yet implemented

Mix a color with all colors of the Widget.

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `color_filter_opa`

Set the intensity of mixing of color filter.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_TRANSP` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `recolor`

Set a color to mix to the Widget.

**Allowed values:** Hexadecimal or RGB values (e.g. '#FF0000', '0xFF0000', 'rgb(255, 0, 0)').

**Default:** `0x000000` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `recolor_opa`

Set the intensity of color mixing.

**Allowed values:**
- `LV_OPA_TRANSP` *(alias: `transparent`)*
- `LV_OPA_0` *(alias: `opa_0`)*
- `LV_OPA_10` *(alias: `opa_10`)*
- `LV_OPA_20` *(alias: `opa_20`)*
- `LV_OPA_30` *(alias: `opa_30`)*
- `LV_OPA_40` *(alias: `opa_40`)*
- `LV_OPA_50` *(alias: `opa_50`)*
- `LV_OPA_60` *(alias: `opa_60`)*
- `LV_OPA_70` *(alias: `opa_70`)*
- `LV_OPA_80` *(alias: `opa_80`)*
- `LV_OPA_90` *(alias: `opa_90`)*
- `LV_OPA_100` *(alias: `opa_100`)*
- `LV_OPA_COVER` *(alias: `cover`)*

**Default:** `LV_OPA_TRANSP` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `anim`

> [!IMPORTANT]
> Not yet implemented

Animation template for Widget's animation. Animation parameters are widget specific.

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `anim_duration`

> [!IMPORTANT]
> Not yet implemented

Animation duration in milliseconds. Its meaning is widget specific.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `transition`

> [!IMPORTANT]
> Not yet implemented

An initialized lv_style_transition_dsc_t to describe a transition.

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `blend_mode`

Describes how to blend the colors to the background.

**Allowed values:**
- `LV_BLEND_MODE_NORMAL` *(alias: `normal`)*
- `LV_BLEND_MODE_ADDITIVE` *(alias: `additive`)*
- `LV_BLEND_MODE_SUBTRACTIVE` *(alias: `subtractive`)*
- `LV_BLEND_MODE_MULTIPLY` *(alias: `multiply`)*
- `LV_BLEND_MODE_DIFFERENCE` *(alias: `difference`)*

**Default:** `LV_BLEND_MODE_NORMAL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `layout`

Set layout of Widget. Children will be repositioned and resized according to the layout's policies.

**Allowed values:**
- `LV_LAYOUT_NONE` *(alias: `none`)*
- `LV_LAYOUT_FLEX` *(alias: `flex`)*
- `LV_LAYOUT_GRID` *(alias: `grid`)*
- `LV_LAYOUT_LAST` *(alias: `last`)*

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `base_dir`

Set base direction of Widget.

**Allowed values:**
- `LV_BASE_DIR_LTR` *(alias: `ltr`)*
- `LV_BASE_DIR_RTL` *(alias: `rtl`)*
- `LV_BASE_DIR_AUTO` *(alias: `auto`)*
- `LV_BASE_DIR_NEUTRAL` *(alias: `neutral`)*
- `LV_BASE_DIR_WEAK` *(alias: `weak`)*

**Default:** `LV_BASE_DIR_AUTO` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `bitmap_mask_src`

> [!IMPORTANT]
> Not yet implemented

If set, a layer will be created for the widget and masked with this A8 bitmap mask.

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `rotary_sensitivity`

Adjust sensitivity for rotary encoders.

**Allowed values:** Integer values in 1/256 unit. 128: half speed, 512: double speed, 256: no change.

**Default:** `256` &nbsp;&nbsp;&nbsp; **Inherited:** Yes &nbsp;&nbsp;&nbsp; **Layout:** No &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Flex

Flex layout properties.

### `flex_flow`

Defines in which direction the flex layout should arrange the children.

**Allowed values:**
- `LV_FLEX_FLOW_ROW` *(alias: `row`)*
- `LV_FLEX_FLOW_COLUMN` *(alias: `column`)*
- `LV_FLEX_FLOW_ROW_WRAP` *(alias: `row_wrap`)*
- `LV_FLEX_FLOW_ROW_REVERSE` *(alias: `row_reverse`)*
- `LV_FLEX_FLOW_ROW_WRAP_REVERSE` *(alias: `row_wrap_reverse`)*
- `LV_FLEX_FLOW_COLUMN_WRAP` *(alias: `column_wrap`)*
- `LV_FLEX_FLOW_COLUMN_REVERSE` *(alias: `column_reverse`)*
- `LV_FLEX_FLOW_COLUMN_WRAP_REVERSE` *(alias: `column_wrap_reverse`)*

**Default:** `LV_FLEX_FLOW_NONE` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `flex_main_place`

Defines how to align the children in the direction of flex flow.

**Allowed values:**
- `LV_FLEX_ALIGN_START` *(alias: `start`)*
- `LV_FLEX_ALIGN_END` *(alias: `end`)*
- `LV_FLEX_ALIGN_CENTER` *(alias: `center`)*
- `LV_FLEX_ALIGN_SPACE_EVENLY` *(alias: `space_evenly`)*
- `LV_FLEX_ALIGN_SPACE_AROUND` *(alias: `space_around`)*
- `LV_FLEX_ALIGN_SPACE_BETWEEN` *(alias: `space_between`)*

**Default:** `LV_FLEX_ALIGN_START` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `flex_cross_place`

Defines how to align the children perpendicular to the direction of flex flow.

**Allowed values:**
- `LV_FLEX_ALIGN_START` *(alias: `start`)*
- `LV_FLEX_ALIGN_END` *(alias: `end`)*
- `LV_FLEX_ALIGN_CENTER` *(alias: `center`)*
- `LV_FLEX_ALIGN_SPACE_EVENLY` *(alias: `space_evenly`)*
- `LV_FLEX_ALIGN_SPACE_AROUND` *(alias: `space_around`)*
- `LV_FLEX_ALIGN_SPACE_BETWEEN` *(alias: `space_between`)*

**Default:** `LV_FLEX_ALIGN_START` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `flex_track_place`

Defines how to align the tracks of the flow.

**Allowed values:**
- `LV_FLEX_ALIGN_START` *(alias: `start`)*
- `LV_FLEX_ALIGN_END` *(alias: `end`)*
- `LV_FLEX_ALIGN_CENTER` *(alias: `center`)*
- `LV_FLEX_ALIGN_SPACE_EVENLY` *(alias: `space_evenly`)*
- `LV_FLEX_ALIGN_SPACE_AROUND` *(alias: `space_around`)*
- `LV_FLEX_ALIGN_SPACE_BETWEEN` *(alias: `space_between`)*

**Default:** `LV_FLEX_ALIGN_START` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `flex_grow`

Defines how much space to take proportionally from the free space of the Widget's track.

**Allowed values:** Integer values. 0 means no grow.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

## Grid

Grid layout properties.

### `grid_column_dsc_array`

An array to describe the columns of the grid.

**Allowed values:** An array of pixel and/or fractional values.
```yaml
grid_column_dsc_array: [10px, 1fr, 2fr]
```

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_column_align`

Defines how to distribute the columns.

**Allowed values:**
- `LV_GRID_ALIGN_START` *(alias: `start`)*
- `LV_GRID_ALIGN_CENTER` *(alias: `center`)*
- `LV_GRID_ALIGN_END` *(alias: `end`)*
- `LV_GRID_ALIGN_STRETCH` *(alias: `stretch`)*
- `LV_GRID_ALIGN_SPACE_EVENLY` *(alias: `space_evenly`)*
- `LV_GRID_ALIGN_SPACE_AROUND` *(alias: `space_around`)*
- `LV_GRID_ALIGN_SPACE_BETWEEN` *(alias: `space_between`)*

**Default:** `LV_GRID_ALIGN_START` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_row_dsc_array`

An array to describe the rows of the grid.

**Allowed values:** An array of pixel and/or fractional values.
```yaml
grid_column_dsc_array: [10px, 1fr, 2fr]
```

**Default:** `NULL` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_row_align`

Defines how to distribute the rows.

**Allowed values:**
- `LV_GRID_ALIGN_START` *(alias: `start`)*
- `LV_GRID_ALIGN_CENTER` *(alias: `center`)*
- `LV_GRID_ALIGN_END` *(alias: `end`)*
- `LV_GRID_ALIGN_STRETCH` *(alias: `stretch`)*
- `LV_GRID_ALIGN_SPACE_EVENLY` *(alias: `space_evenly`)*
- `LV_GRID_ALIGN_SPACE_AROUND` *(alias: `space_around`)*
- `LV_GRID_ALIGN_SPACE_BETWEEN` *(alias: `space_between`)*

**Default:** `LV_GRID_ALIGN_START` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_cell_column_pos`

Set column in which Widget should be placed.

**Allowed values:** Integer values >= 0.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_cell_x_align`

Set how to align Widget horizontally within its grid cell.

**Allowed values:**
- `LV_GRID_ALIGN_START` *(alias: `start`)*
- `LV_GRID_ALIGN_CENTER` *(alias: `center`)*
- `LV_GRID_ALIGN_END` *(alias: `end`)*
- `LV_GRID_ALIGN_STRETCH` *(alias: `stretch`)*
- `LV_GRID_ALIGN_SPACE_EVENLY` *(alias: `space_evenly`)*
- `LV_GRID_ALIGN_SPACE_AROUND` *(alias: `space_around`)*
- `LV_GRID_ALIGN_SPACE_BETWEEN` *(alias: `space_between`)*

**Default:** `LV_GRID_ALIGN_START` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_cell_column_span`

Set how many columns Widget should span.

**Allowed values:** Integer values >= 1.

**Default:** `1` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_cell_row_pos`

Set row in which Widget should be placed.

**Allowed values:** Integer values >= 0.

**Default:** `0` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_cell_y_align`

Set how to align Widget vertically within its grid cell.

**Allowed values:**
- `LV_GRID_ALIGN_START` *(alias: `start`)*
- `LV_GRID_ALIGN_CENTER` *(alias: `center`)*
- `LV_GRID_ALIGN_END` *(alias: `end`)*
- `LV_GRID_ALIGN_STRETCH` *(alias: `stretch`)*
- `LV_GRID_ALIGN_SPACE_EVENLY` *(alias: `space_evenly`)*
- `LV_GRID_ALIGN_SPACE_AROUND` *(alias: `space_around`)*
- `LV_GRID_ALIGN_SPACE_BETWEEN` *(alias: `space_between`)*

**Default:** `LV_GRID_ALIGN_START` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---

### `grid_cell_row_span`

Set how many rows Widget should span.

**Allowed values:** Integer values >= 1.

**Default:** `1` &nbsp;&nbsp;&nbsp; **Inherited:** No &nbsp;&nbsp;&nbsp; **Layout:** Yes &nbsp;&nbsp;&nbsp; **Ext. draw:** No

---
