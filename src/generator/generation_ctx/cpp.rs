use super::base_ctx::BaseCtx;
use super::language_ctx::LanguageCtx;

pub struct Cpp;

impl LanguageCtx for Cpp {
    fn templates() -> &'static [(&'static str, &'static str)] {
        &[
            (
                "styles_header",
                include_str!("../tera_templates/cpp/styles.h.tera"),
            ),
            (
                "stylesheets_header",
                include_str!("../tera_templates/cpp/stylesheets.h.tera"),
            ),
            (
                "stylesheets_source",
                include_str!("../tera_templates/cpp/stylesheets.cpp.tera"),
            ),
            (
                "stylesheet_header",
                include_str!("../tera_templates/cpp/stylesheet.h.tera"),
            ),
            (
                "stylesheet_source",
                include_str!("../tera_templates/cpp/stylesheet.cpp.tera"),
            ),
            #[cfg(feature = "macros")]
            (
                "macros",
                include_str!("../tera_templates/cpp/macros.h.tera"),
            ),
        ]
    }

    fn source_extension() -> &'static str {
        "cpp"
    }

    fn setup(base: &mut BaseCtx, namespace: Option<&str>) {
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

        if let Some(ns) = namespace {
            base.insert_namespace(ns);
        }
    }
}
