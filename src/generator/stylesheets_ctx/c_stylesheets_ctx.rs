use crate::generator::filters::filters;

use super::stylesheet_ctx::StyleSheetsContext;

pub struct CStyleSheetsContext<'a> {
    pub base: &'a StyleSheetsContext,
    pub tera: tera::Tera,
}

impl<'a> CStyleSheetsContext<'a> {
    pub fn from(base: &'a mut StyleSheetsContext) -> Result<Self, String> {
        let mut tera = tera::Tera::default();

        filters::apply_filters(&mut tera);

        tera.add_raw_template(
            "styles_header",
            include_str!("../templates/c/styles.h.tera"),
        )
        .map_err(|e| {
            format!(
                "An error occurred while loading the Tera template 'styles.h.tera': {}",
                e
            )
        })?;
        tera.add_raw_template(
            "stylesheets_header",
            include_str!("../templates/c/stylesheets.h.tera"),
        )
        .map_err(|e| {
            format!(
                "An error occurred while loading the Tera template 'stylesheets.h.tera': {}",
                e
            )
        })?;
        tera.add_raw_template(
            "stylesheets_source",
            include_str!("../templates/c/stylesheets.c.tera"),
        )
        .map_err(|e| {
            format!(
                "An error occurred while loading the Tera template 'stylesheets.c.tera': {}",
                e
            )
        })?;
        tera.add_raw_template(
            "stylesheet_header",
            include_str!("../templates/c/stylesheet.h.tera"),
        )
        .map_err(|e| {
            format!(
                "An error occurred while loading the Tera template 'stylesheet.h.tera': {}",
                e
            )
        })?;
        tera.add_raw_template(
            "stylesheet_source",
            include_str!("../templates/c/stylesheet.c.tera"),
        )
        .map_err(|e| {
            format!(
                "An error occurred while loading the Tera template 'stylesheet.c.tera': {}",
                e
            )
        })?;

        base.stylesheets_helper.source.path.set_extension("c");
        for stylesheet in &mut base.stylesheets {
            stylesheet.source.path.set_extension("c");
        }

        Ok(Self { base, tera })
    }
}
