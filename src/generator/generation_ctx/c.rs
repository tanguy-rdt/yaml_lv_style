use super::base_ctx::BaseCtx;
use super::language_ctx::LanguageCtx;

pub struct C;

impl LanguageCtx for C {
    fn templates() -> &'static [(&'static str, &'static str)] {
        &[
            (
                "styles_header",
                include_str!("../tera_templates/c/styles.h.tera"),
            ),
            (
                "stylesheets_header",
                include_str!("../tera_templates/c/stylesheets.h.tera"),
            ),
            (
                "stylesheets_source",
                include_str!("../tera_templates/c/stylesheets.c.tera"),
            ),
            (
                "stylesheet_header",
                include_str!("../tera_templates/c/stylesheet.h.tera"),
            ),
            (
                "stylesheet_source",
                include_str!("../tera_templates/c/stylesheet.c.tera"),
            ),
            #[cfg(feature = "macros")]
            ("macros", include_str!("../tera_templates/c/macros.h.tera")),
        ]
    }

    fn source_extension() -> &'static str {
        "c"
    }

    fn setup(base: &mut BaseCtx, _namespace: Option<&str>) {
        base.stylesheets_helper
            .source
            .path
            .set_extension(Self::source_extension());
        for stylesheet in &mut base.stylesheets {
            stylesheet
                .source
                .path
                .set_extension(Self::source_extension());
        }
    }
}
