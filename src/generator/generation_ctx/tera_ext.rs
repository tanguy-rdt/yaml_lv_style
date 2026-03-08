pub trait TeraContextExt {
    fn insert_or_err(&mut self, key: &str, value: &impl serde::Serialize) -> Result<(), String>;
}

impl TeraContextExt for tera::Context {
    fn insert_or_err(&mut self, key: &str, value: &impl serde::Serialize) -> Result<(), String> {
        self.try_insert(key, value).map_err(|e| format!("{e}"))
    }
}

pub trait TeraExt {
    fn add_raw_template_or_err(&mut self, name: &str, content: &str) -> Result<(), String>;
}

impl TeraExt for tera::Tera {
    fn add_raw_template_or_err(&mut self, name: &str, content: &str) -> Result<(), String> {
        self.add_raw_template(name, content)
            .map_err(|e| format!("{e}"))
    }
}
