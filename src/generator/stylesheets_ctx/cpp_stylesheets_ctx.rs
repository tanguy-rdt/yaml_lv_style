use crate::generator::filters::filters;

use super::stylesheet_ctx::StyleSheetsContext;

pub struct CppStyleSheetsContext<'a> {
    pub base: &'a StyleSheetsContext,
    pub tera: tera::Tera
}

impl<'a> CppStyleSheetsContext<'a> {
    pub fn from(base: &'a mut StyleSheetsContext, namespace: Option<&str>) -> Result<Self, String> {
        let mut tera = tera::Tera::default();

        filters::apply_filters(&mut tera);

        tera.add_raw_template("styles_header", include_str!("../templates/cpp/styles.h.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'styles.h.tera': {}", e))?;
        tera.add_raw_template("stylesheets_header", include_str!("../templates/cpp/stylesheets.h.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheets.h.tera': {}", e))?;
        tera.add_raw_template("stylesheets_source", include_str!("../templates/cpp/stylesheets.cpp.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheets.cpp.tera': {}", e))?;
        tera.add_raw_template("stylesheet_header", include_str!("../templates/cpp/stylesheet.h.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheet.h.tera': {}", e))?;
        tera.add_raw_template("stylesheet_source", include_str!("../templates/cpp/stylesheet.cpp.tera"))
            .map_err(|e| format!("An error occurred while loading the Tera template 'stylesheet.cpp.tera': {}", e))?;

        base.stylesheets_helper.source.path.set_extension("cpp");
        for stylesheet in &mut base.stylesheets {
            stylesheet.source.path.set_extension("cpp"); 
        }
        
        if let Some(namespace) = namespace {
            base.stylesheets_helper.source.tera_context.insert("namespace", namespace);
            base.stylesheets_helper.header.tera_context.insert("namespace", namespace);
            for stylesheet in &mut base.stylesheets {
                stylesheet.source.tera_context.insert("namespace", namespace);
                stylesheet.header.tera_context.insert("namespace", namespace);
            }
        }

        Ok(Self { base, tera })
    }
}