#[derive(Debug, Clone, PartialEq, Eq, clap::ValueEnum)]
pub enum ClangFormatStyle {
    Llvm,
    Gnu,
    Google,
    Chromium,
    Microsoft,
    Mozilla,
    WebKit,
    File,
}

impl ClangFormatStyle {
    pub fn to_clang_preset(&self) -> &str {
        match self {
            Self::Llvm => "LLVM",
            Self::Google => "Google",
            Self::Chromium => "Chromium",
            Self::Mozilla => "Mozilla",
            Self::WebKit => "WebKit",
            Self::Microsoft => "Microsoft",
            Self::Gnu => "GNU",
            Self::File => "file",
        }
    }
}
