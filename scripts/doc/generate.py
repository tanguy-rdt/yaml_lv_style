import tomllib
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
PROPS_FILE = SCRIPT_DIR / "toml/properties.toml"
TYPES_FILE = SCRIPT_DIR / "toml/types.toml"
OUTPUT_FILE = SCRIPT_DIR / "../../doc/style_properties.md"

NOTE_LEVELS = {
    "note_info":      "NOTE",
    "note_tip":       "TIP",
    "note_important": "IMPORTANT",
    "note_warning":   "WARNING",
    "note_caution":   "CAUTION",
}

SKIP_KEYS = {"default", "inherited", "layout", "ext_draw"}


def load_toml(path: Path) -> dict:
    with open(path, "rb") as f:
        return tomllib.load(f)


def is_type_ref(value: str, types: dict) -> bool:
    return value in types


def render_allowed_values(allowed_values: str, types: dict) -> str:
    lines = []

    if is_type_ref(allowed_values, types):
        type_def = types[allowed_values]
        values = type_def.get("allowed_values", [])
        if isinstance(values, list):
            lines.append("**Allowed values:**")
            for v in values:
                lines.append(f"- `{v['name']}` *(alias: `{v['alias']}`)*")
        else:
            lines.append(f"**Allowed values:** {values}")

        example = type_def.get("example")
        if example:
            lines.append("```yaml")
            lines.append(example.strip())
            lines.append("```")
    else:
        lines.append(f"**Allowed values:** {allowed_values}")

    return "\n".join(lines)


def render_prop(prop_name: str, prop: dict, types: dict) -> str:
    lines = []

    lines.append(f"### `{prop_name}`")
    lines.append("")

    for key, value in prop.items():
        if key in SKIP_KEYS:
            continue

        if key in NOTE_LEVELS:
            lines.append(f"> [!{NOTE_LEVELS[key]}]")
            lines.append(f"> {value}")
            lines.append("")

        elif key == "desc":
            lines.append(value)
            lines.append("")

        elif key == "allowed_values":
            lines.append(render_allowed_values(value, types))
            lines.append("")

    default = prop.get("default", "—")
    inherited = "Yes" if prop.get("inherited") else "No"
    layout = "Yes" if prop.get("layout") else "No"
    ext_draw = "Yes" if prop.get("ext_draw") else "No"

    lines.append(
        f"**Default:** `{default}` &nbsp;&nbsp;&nbsp; "
        f"**Inherited:** {inherited} &nbsp;&nbsp;&nbsp; "
        f"**Layout:** {layout} &nbsp;&nbsp;&nbsp; "
        f"**Ext. draw:** {ext_draw}"
    )
    lines.append("")
    lines.append("---")
    lines.append("")

    return "\n".join(lines)


def generate_toc(props: dict) -> str:
    lines = []

    lines.append("## Table of Contents")
    lines.append("")

    for section_name, section in props.items():
        if not isinstance(section, dict):
            continue

        section_title = section_name.replace('_', ' ').title()
        section_anchor = section_name.replace('_', '-').lower()
        lines.append(f"- [{section_title}](#{section_anchor})")

        for key, value in section.items():
            if key == "desc":
                continue
            if isinstance(value, dict):
                if "note_important" in value:
                    lines.append(f"  - :heavy_exclamation_mark: [`{key}`](#{key}) — Not yet implemented")
                else:
                    desc = value.get("desc", "")
                    short_desc = desc.split('.')[0] if desc else ""
                    lines.append(f"  - [`{key}`](#{key}) — {short_desc}")

    lines.append("")
    return "\n".join(lines)


def generate_readme(props: dict, types: dict) -> str:
    lines = []

    lines.append("# Style Properties")
    lines.append("")
    lines.append("> [!NOTE]")
    lines.append("> This documentation is inspired by the official [LVGL documentation: Style Properties](https://docs.lvgl.io/master/common-widget-features/styles/style-properties.html#).")
    lines.append("")

    lines.append(generate_toc(props))

    for section_name, section in props.items():
        if not isinstance(section, dict):
            continue

        section_desc = section.get("desc")
        if section_desc:
            lines.append(f"## {section_name.replace('_', ' ').title()}")
            lines.append("")
            lines.append(section_desc)
            lines.append("")

        for key, value in section.items():
            if key == "desc":
                continue
            if isinstance(value, dict):
                lines.append(render_prop(key, value, types))

    return "\n".join(lines)


def main():
    props = load_toml(PROPS_FILE)
    types = load_toml(TYPES_FILE)

    readme = generate_readme(props, types)

    with open(OUTPUT_FILE, "w", encoding="utf-8") as f:
        f.write(readme)

    print(f"Generated {OUTPUT_FILE}")


if __name__ == "__main__":
    main()