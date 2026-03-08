use super::base_ctx::BaseCtx;

pub trait LanguageCtx {
    fn templates() -> &'static [(&'static str, &'static str)];
    fn source_extension() -> &'static str;
    fn setup(base: &mut BaseCtx, namespace: Option<&str>);
}
