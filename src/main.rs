mod stylesheet;

use crate::stylesheet::stylesheet::StyleSheet;

fn main() {
    env_logger::init();

    match StyleSheet::from_yaml("example/style.yaml") {
        Ok(stylesheet) =>  print_styles(&stylesheet),
        Err(e) => {
            log::error!("{}", e);
            std::process::exit(1);
        }
    }
}

fn print_styles(styles: &StyleSheet) {
    for style in &styles.styles {
        println!("Style: {}", style.name);

        let state_fields = [
            ("default", &style.default),
            ("checked", &style.checked),
            ("focused", &style.focused),
            ("focus_key", &style.focus_key),
            ("edited", &style.edited),
            ("hovered", &style.hovered),
            ("pressed", &style.pressed),
            ("disabled", &style.disabled),
            ("user_1", &style.user_1),
            ("user_2", &style.user_2),
            ("user_3", &style.user_3),
            ("user_4", &style.user_4),
            ("any", &style.any),
        ];

        for (state_name, maybe_props) in state_fields {
            if let Some(props) = maybe_props {
                println!("  State: {}", state_name);

                let value: yaml_serde::Value = yaml_serde::to_value(props).unwrap();

                if let yaml_serde::Value::Mapping(map) = value {
                    for (k, v) in map {
                        if let yaml_serde::Value::Null = v {
                            continue; // ignore les None
                        }

                        let serialized = yaml_serde::to_string(&v)
                            .unwrap()
                            .trim()
                            .to_string();

                        if let yaml_serde::Value::String(k_str) = k {
                            println!("    {}: {}", k_str, serialized);
                        } else {
                            log::warn!("Clé non string dans props?");
                        }
                    }
                }
            }
        }
    }
}