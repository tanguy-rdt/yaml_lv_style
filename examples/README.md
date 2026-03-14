# Examples

The following examples demonstrate how to use `yaml_lv_style` in a real LVGL project.

| Example | Language | Description |
|---------|----------|-------------|
| [simple_c](examples/simple_c/) | C | Theme switching with dynamic styles |
| [simple_cpp](examples/simple_cpp/) | C++ | Theme switching with dynamic and constant styles |

---

## Running the examples

All examples use CMake and require LVGL with the SDL driver.

**Configure and build:**

```bash
cd examples
cmake --preset yaml_lv_style_examples
cmake --build --preset yaml_lv_style_examples
```

**Run an example:**

```bash
./cmake-build-release/examples/simple_c/simple_c_example
```