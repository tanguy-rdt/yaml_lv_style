# Usage

## Table of Contents

- [YAML Format](#yaml-format)
- [Generated Files](#generated-files)
- [C++ Usage](#c-usage)
- [C Usage](#c-usage-1)
- [CMake](#cmake)
- [CLI](#cli)

---

## YAML Format

A YAML file defines one or more styles. Each style has a name, a `const` flag, and one or more selectors.

> [!TIP]
> Everywhere an LVGL enum is expected (selectors, parts, and property values), both the full LVGL constant and a short
> snake_case alias are accepted.

```yaml
name: my_stylesheet

styles:
  - name: button
    const: true
    default:
      bg_color: "#3B82F6"
      text_color: "#F1F5F9"
      align: center
    hovered:
      bg_color: "#2563EB"

  - name: label
    const: false
    default:
      text_color: "#1E293B"
      text_align: center
```

### `name`

The optional top-level `name` field sets the name of the stylesheet. If omitted, the name of the YAML file is used.

### `const`

- `true` — the style is stored in ROM via `LV_STYLE_CONST_INIT`. No runtime initialization required.
- `false` — the style is a dynamic style, initialized at runtime. `initStyleSheets()` must be called before use.

> [!NOTE]
> If `const` field is not specified, it defaults to `false`. 
> In C++ output, `const: true` styles are generated as `constexpr`.

### Selectors

A selector defines which state and/or part the properties apply to. It follows the format `state` or `state.part`:

```yaml
styles:
  - name: slider
    const: true
    default:            # LV_STATE_DEFAULT | LV_PART_MAIN
      bg_color: "#1E293B"
    default.indicator:  # LV_STATE_DEFAULT | LV_PART_INDICATOR
      bg_color: "#3B82F6"
    hovered.knob:       # LV_STATE_HOVERED | LV_PART_KNOB
      bg_color: "#2563EB"
```

**Supported states:**
- `LV_STATE_DEFAULT` *(alias: `default`)* \
  *Normal, released state.*
- `LV_STATE_CHECKED` *(alias: `checked`)* \
  *Toggled or checked state.*
- `LV_STATE_FOCUSED` *(alias: `focused`)* \
  *Focused via keypad, encoder, or clicked via touchpad/mouse.*
- `LV_STATE_FOCUS_KEY` *(alias: `focus_key`)* \
  *Focused via keypad or encoder but not via touchpad/mouse.*
- `LV_STATE_EDITED` *(alias: `edited`)* \
  *Being edited by an encoder.*
- `LV_STATE_HOVERED` *(alias: `hovered`)* \
  *Hovered by a mouse.*
- `LV_STATE_PRESSED` *(alias: `pressed`)* \
  *Being pressed.*
- `LV_STATE_DISABLED` *(alias: `disabled`)* \
  *Being scrolled.*
- `LV_STATE_USER_1` *(alias: `user_1`)* \
  *Custom state.*
- `LV_STATE_USER_2` *(alias: `user_2`)* \
  *Custom state.*
- `LV_STATE_USER_3` *(alias: `user_3`)* \
  *Custom state.*
- `LV_STATE_USER_4` *(alias: `user_4`)* \
  *Custom state.*
- `LV_STATE_ANY` *(alias: `any`)* 

**Supported parts:**
- `LV_PART_MAIN` *(alias: `main`)* \
  *A background-like rectangle.*
- `LV_PART_SCROLLBAR` *(alias: `scrollbar`)* \
  *The scrollbar(s).*
- `LV_PART_INDICATOR` *(alias: `indicator`)* \
  *The indicator, e.g., for sliders, bars, switches, or the tick box of a checkbox.*
- `LV_PART_KNOB` *(alias: `knob`)* \
  *A handle used to adjust a value.*
- `LV_PART_SELECTED` *(alias: `selected`)* \
  *Indicates the currently selected option or section.*
- `LV_PART_ITEMS` *(alias: `items`)* \
  *Used if the widget has multiple similar elements (e.g., table cells).*
- `LV_PART_CURSOR` *(alias: `cursor`)* \
  *Marks a specific position, e.g., the cursor in a text area or chart.*
- `LV_PART_CUSTOM_FIRST` *(alias: `custom_1`)* \
  *Custom parts can be added starting from here.*
- `LV_PART_ANY` *(alias: `any`)*

### Properties

> [!TIP]
> Properties correspond directly to LVGL's `lv_style_set_*` functions. 

All supported properties and their accepted values are listed in [style_properties.md](style_properties.md). 
The [official LVGL style documentation](https://docs.lvgl.io/9.5/common-widget-features/styles/style-properties.html) 
is also a valid reference.

### Multiple stylesheets

Multiple YAML files can be passed to the generator. Each file produces its own stylesheet.

---

## Generated Files

For a stylesheet file named `button.yaml`, the following files are generated:

| File | Description |
|------|-------------|
| `styles.h` | Contains one enum type per stylesheet, each listing the style names it defines |
| `stylesheets.h` / `stylesheets.cpp` | Helpers to initialize and apply styles across all stylesheets |
| `stylesheet_button.h` / `stylesheet_button.cpp` | Stylesheet-specific implementation |

---

## C++ Usage

Include `stylesheets.h` and `styles.h`:

```cpp
#include "styles_gen/stylesheets.h"
#include "styles_gen/styles.h"
```

If any dynamic style is used, call `initStyleSheets()` before applying any style:

```cpp
initStyleSheets();
```

Apply a style to an object with `setStyle`:

```cpp
lv_obj_t* btn = lv_button_create(lv_screen_active());
setStyle(btn, ButtonStyle::Button);
```

For dynamic styles, a getter is also available:

```cpp
lv_style_t* style = getStyle(ButtonStyle::Button, LV_PART_MAIN | LV_STATE_DEFAULT);
```

---

## C Usage

Include `stylesheets.h` and `styles.h`:

```c
#include "styles_gen/stylesheets.h"
#include "styles_gen/styles.h"
```

If any dynamic style is used, call `init_style_sheets()` before applying any style:

```c
init_style_sheets();
```

> [!IMPORTANT]
> In C, `setStyle` is not available since function overloading is not supported. Include the specific stylesheet header 
> and use the dedicated function instead:

```c
#include "styles_gen/stylesheet_button.h"

lv_obj_t* btn = lv_button_create(lv_screen_active());
set_button_style(btn, BUTTON_STYLE_BUTTON);
```

For dynamic styles, a getter is also available:

```c
lv_style_t* style = get_button_style(BUTTON_STYLE_BUTTON, LV_PART_MAIN | LV_STATE_DEFAULT);
```

---

## CMake

Add the helper script to your project:

```cmake
include(cmake/get-yaml_lv_style.cmake)
get_yaml_lv_style(latest) # or a specific tag: get_yaml_lv_style(v1.0.0)
```

Two functions are available depending on the target language:

- `yaml_lv_style_generate_cpp` — generates C++ stylesheets
- `yaml_lv_style_generate_c` — generates C stylesheets

Both functions accept the following options:

| Option        | Description                                                                                                |
|---------------|------------------------------------------------------------------------------------------------------------|
| `FILES`       | YAML style files to process                                                                                |
| `OUTPUT_DIR`  | Directory where the generated files will be written                                                        |
| `ALIAS`       | Alias for the generated targets                                                                            |
| `FORMAT`      | clang-format style to apply to generated code. Defaults to `file` (uses `.clang-format` from the project). |
| `NAMESPACE`   | Namespace for generated code (C++ only)                                                                    |

```cmake
yaml_lv_style_generate_cpp(my_styles
    FILES
        styles/button.yaml
        styles/label.yaml
    OUTPUT_DIR ${CMAKE_CURRENT_BINARY_DIR}/styles_gen
    NAMESPACE ui::styles
)
```

Two targets are generated:

- `my_styles::my_styles` — full stylesheet library, depends publicly on LVGL, includes `style_names`
- `my_styles::style_names` — style enums only, without the LVGL dependency

If an `ALIAS` is provided, the targets are named accordingly:
- `<alias>` — full stylesheet library
- `<alias>_names` — style enums only

```cmake
# Link the full stylesheet
target_link_libraries(my_app PRIVATE my_styles::my_styles)

# Or link only the enums, without pulling in LVGL
target_link_libraries(my_app PRIVATE my_styles::style_names)

# With a custom alias
target_link_libraries(my_app PRIVATE styles::my_styles)
target_link_libraries(my_app PRIVATE styles::my_styles_names)
```

---

## CLI

`yaml_lv_style` is a command-line tool that takes one or more YAML files as input and generates the corresponding C or 
C++ stylesheets. The output directory, target language, clang-format style, and an optional C++ namespace can all be 
configured.

```
Usage: yaml_lv_style [OPTIONS] --input <FILE>... --language <LANGUAGE>
 
Options:
  -i, --input <FILE>...      Path to the input YAML stylesheet files to be processed.
  -o, --output-dir <DIR>     Directory where the generated files will be saved (defaults to current directory).
                              [default: .]
  -l, --language <LANGUAGE>  Language for the generated files.
                             Possible values: [c, cpp]
  -f, --format [<FORMAT>]    Format the generated code with clang-format (if available on your system).
                             Default value: file
                             Possible values: [llvm, gnu, google, chromium, microsoft, mozilla, webkit, file]
  -n, --namespace <NAME>     Optional C++ namespace to wrap the generated code (e.g., 'ui::styles').
      --output-list <FILE>   Write a list of all generated files to the specified file.
  -h, --help                 Print help
  -V, --version              Print version
```