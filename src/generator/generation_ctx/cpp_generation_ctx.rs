use crate::generator::generation_ctx::GenerationCtx;
use crate::generator::tera_filters;

pub struct CppGenerationCtx {
    pub tera: tera::Tera,
}

impl<'a> CppGenerationCtx {
    pub fn from(base: &'a mut GenerationCtx, namespace: Option<&str>) -> Result<Self, String> {
        let mut tera = tera::Tera::default();

        tera_filters::apply_filters(&mut tera);

        tera.add_raw_template(
            "styles_header",
            include_str!("../tera_templates/cpp/styles.h.tera"),
        )
        .map_err(|e| format!("{e}"))?;
        tera.add_raw_template(
            "stylesheets_header",
            include_str!("../tera_templates/cpp/stylesheets.h.tera"),
        )
        .map_err(|e| format!("{e}"))?;
        tera.add_raw_template(
            "stylesheets_source",
            include_str!("../tera_templates/cpp/stylesheets.cpp.tera"),
        )
        .map_err(|e| format!("{e}"))?;
        tera.add_raw_template(
            "stylesheet_header",
            include_str!("../tera_templates/cpp/stylesheet.h.tera"),
        )
        .map_err(|e| format!("{e}"))?;
        tera.add_raw_template(
            "stylesheet_source",
            include_str!("../tera_templates/cpp/stylesheet.cpp.tera"),
        )
        .map_err(|e| format!("{e}"))?;

        base.stylesheets_helper.source.path.set_extension("cpp");
        for stylesheet in &mut base.stylesheets {
            stylesheet.source.path.set_extension("cpp");
        }

        if let Some(namespace) = namespace {
            base.stylesheets_helper
                .source
                .tera_context
                .insert("namespace", namespace);
            base.stylesheets_helper
                .header
                .tera_context
                .insert("namespace", namespace);
            for stylesheet in &mut base.stylesheets {
                stylesheet
                    .source
                    .tera_context
                    .insert("namespace", namespace);
                stylesheet
                    .header
                    .tera_context
                    .insert("namespace", namespace);
            }
        }

        Ok(Self { tera })
    }
}
