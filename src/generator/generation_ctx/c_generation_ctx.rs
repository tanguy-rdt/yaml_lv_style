use crate::generator::generation_ctx::GenerationCtx;
use crate::generator::tera_filters;

pub struct CGenerationCtx {
    pub tera: tera::Tera,
}

impl<'a> CGenerationCtx {
    pub fn from(base: &'a mut GenerationCtx) -> Result<Self, String> {
        let mut tera = tera::Tera::default();

        tera_filters::apply_filters(&mut tera);

        tera.add_raw_template(
            "styles_header",
            include_str!("../tera_templates/c/styles.h.tera"),
        )
        .map_err(|e| format!("{e}"))?;

        tera.add_raw_template(
            "stylesheets_header",
            include_str!("../tera_templates/c/stylesheets.h.tera"),
        )
        .map_err(|e| format!("{e}"))?;

        tera.add_raw_template(
            "stylesheets_source",
            include_str!("../tera_templates/c/stylesheets.c.tera"),
        )
        .map_err(|e| format!("{e}"))?;

        tera.add_raw_template(
            "stylesheet_header",
            include_str!("../tera_templates/c/stylesheet.h.tera"),
        )
        .map_err(|e| format!("{e}"))?;

        tera.add_raw_template(
            "stylesheet_source",
            include_str!("../tera_templates/c/stylesheet.c.tera"),
        )
        .map_err(|e| format!("{e}"))?;

        base.stylesheets_helper.source.path.set_extension("c");
        for stylesheet in &mut base.stylesheets {
            stylesheet.source.path.set_extension("c");
        }

        Ok(Self { tera })
    }
}
