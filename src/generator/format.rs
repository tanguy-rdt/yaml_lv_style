use serde::Deserialize;

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
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
    pub fn to_clang_preset(self) -> String {
        match self {
            Self::Llvm => "LLVM".to_string(),
            Self::Google => "Google".to_string(),
            Self::Chromium => "Chromium".to_string(),
            Self::Mozilla => "Mozilla".to_string(),
            Self::WebKit => "WebKit".to_string(),
            Self::Microsoft => "Microsoft".to_string(),
            Self::Gnu => "GNU".to_string(),
            Self::File => "file".to_string(),
        }
    }
}
