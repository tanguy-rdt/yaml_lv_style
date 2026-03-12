# yaml_lv_style

Writing LVGL styles in C or C++ is verbose and repetitive. `yaml_lv_style` lets you define your styles in a clean YAML 
file and generates all the boilerplate for you.

`yaml_lv_style` supports both constant styles (stored in ROM) and dynamic styles, outputs C and C++ code, handles 
multiple selectors per style combining any state and part, and integrates into CMake projects. Compatible with LVGL 9.5.

Define your styles in a YAML file:

```yaml
styles:
  - name: button
    const: true
    default:
      align: center
      bg_color: "#3B82F6"
      text_color: "#F1F5F9"
    hovered:
      bg_color: "#2563EB"
```

Once generated, applying a style to any LVGL object is as simple as:

```cpp
#include "styles_gen/stylesheets.h"
#include "styles_gen/styles.h"

lv_obj_t* btn = lv_button_create(lv_screen_active());
setStyle(btn, ButtonStyle::Button);
```

That's it.


## Getting Started

`yaml_lv_style` can be integrated into any CMake project via a lightweight helper script that automatically downloads 
the right release, exposes functions to convert your YAML files into C or C++ stylesheets, and creates a ready-to-use 
target that bundles everything together. See the [CMake](doc/usage.md#cmake) section.

The YAML format supports multiple styles, selectors, states, and parts. All available properties are listed in 
[doc/style_properties.md](doc/style_properties.md). See the [YAML format](doc/usage.md#yaml-format) section for the 
full reference.

Once the stylesheets are generated, applying a style to any LVGL object takes a single line. See the 
[C++](doc/usage.md#c-usage) and [C](doc/usage.md#c-usage-1) usage sections.

Prebuilt binaries are also available on the [releases page](https://github.com/tanguy-rdt/yaml_lv_style/releases) 
for Linux (`.deb` and `.tar.gz`) and macOS, for those who prefer to use the CLI directly. 
See the [CLI](doc/usage.md#cli) section.


## Documentation

- [doc/usage.md](doc/usage.md) — YAML format, generated files, C/C++ and CMake usage
- [doc/style_properties.md](doc/style_properties.md) — all supported properties and accepted values
- [examples/](examples/) — ready-to-use examples