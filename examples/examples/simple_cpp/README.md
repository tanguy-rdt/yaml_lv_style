# simple_cpp

A simple theme switcher written in C++. Clicking the button toggles between a light and a dark theme, demonstrating how to use multiple stylesheets and apply them at runtime using the C++ API.

<p align="center">
  <img src="../../assets/example_simple.gif" width="300">
</p>

## What it demonstrates

- Generating multiple stylesheets from separate YAML files (`light.yaml`, `dark.yaml`)
- Using dynamic styles (`const: false`) initialized at runtime via `initStyleSheets()`
- Using constant styles (`const: true`) stored in ROM via `LV_STYLE_CONST_INIT`
- Applying styles in C++ using the unified `setStyle` function
- Switching styles at runtime on a button click event

## Build and run

```bash
cd examples
cmake --preset yaml_lv_style_examples
cmake --build --preset yaml_lv_style_examples
./cmake-build-release/examples/simple_cpp/simple_cpp_example
```