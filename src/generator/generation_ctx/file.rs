use std::path::PathBuf;

pub struct FileCtx {
    pub tera_template: String,
    pub tera_context: tera::Context,
    pub path: PathBuf,
}

pub struct Component {
    pub source: FileCtx,
    pub header: FileCtx,
}
