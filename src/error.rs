#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildToolError {
    MissingRustNightly, // Rust nightly 工具链未安装
    RustupUnavailable,  // rustup 不可使用/执行
                        // UnknownError(String),
}

impl std::error::Error for BuildToolError {}

impl std::fmt::Display for BuildToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildToolError::MissingRustNightly => {
                write!(f, "Rust nightly toolchain is required but not installed")
            }
            // BuildToolError::UnknownError(s) => {
            //     write!(f, "Unknown error occurred: {}", s)
            // }
            BuildToolError::RustupUnavailable => {
                write!(f, "rustup is not available or cannot be executed")
            }
        }
    }
}
