use std::error::Error;
use std::fmt;
use std::process::{Command, Stdio};

// 错误类型定义
#[derive(Debug)]
pub enum DepCheckError {
    ToolMissing(String),
    CargoTomlNotFound,
    TomlParseError(String),
    DependencyNotFound(String),
    CommandFailed(String),
}

impl fmt::Display for DepCheckError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ToolMissing(msg) => write!(f, "Tool missing: {msg}"),
            Self::CargoTomlNotFound => write!(f, "Cargo.toml file not found"),
            Self::TomlParseError(msg) => write!(f, "TOML parse error: {msg}"),
            Self::DependencyNotFound(dep) => write!(f, "Dependency not found: {dep}"),
            Self::CommandFailed(msg) => write!(f, "Command execution failed: {msg}"),
        }
    }
}

impl Error for DepCheckError {}

// 常量定义
pub const CARGO_TOML: &str = "Cargo.toml";
pub const UDEPS_CMD: &[&str] = &["cargo", "+nightly", "udeps", "--all-targets"];

// 依赖位置信息
pub struct DependencyLocation {
    pub section: String,
    pub flag: Option<String>,
}

// 移除结果
pub struct RemovalResult {
    pub success: bool,
    pub message: String,
}

// 现有工具检查函数保持不变
pub fn check_command(cmd: &str) -> Result<(), Box<dyn Error>> {
    #[cfg(target_os = "windows")]
    let check_cmd = "where";
    #[cfg(not(target_os = "windows"))]
    let check_cmd = "which";

    Command::new(check_cmd)
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    Ok(())
}

pub fn check_rust_nightly() -> Result<(), Box<dyn Error>> {
    let output = Command::new("rustup")
        .args(["run", "nightly", "rustc", "--version"])
        .output()?;

    let stdout = String::from_utf8(output.stdout)?;
    if !stdout.contains("nightly") {
        return Err("Rust nightly toolchain is required".into());
    }

    Ok(())
}

pub fn check_upx_lzma() -> Result<(), Box<dyn Error>> {
    let output = Command::new("upx").args(["--help"]).output()?;

    let stdout = String::from_utf8(output.stdout)?;
    if !stdout.contains("--lzma") {
        return Err("UPX with LZMA support is required".into());
    }

    Ok(())
}

// 加载Cargo.toml内容
pub fn load_cargo_toml() -> Result<String, DepCheckError> {
    std::fs::read_to_string(CARGO_TOML).map_err(|_| DepCheckError::CargoTomlNotFound)
}

// 解析Cargo.toml
pub fn parse_cargo_toml(content: &str) -> Result<toml::Value, DepCheckError> {
    content
        .parse::<toml::Value>()
        .map_err(|e| DepCheckError::TomlParseError(e.to_string()))
}

// 移除依赖项
pub fn remove_dependency(dep: &str, location: &DependencyLocation) -> RemovalResult {
    let mut cmd = Command::new("cargo");
    cmd.arg("remove").arg(dep);

    if let Some(flag) = &location.flag {
        cmd.arg(flag);
    }

    match cmd.output() {
        Ok(output) if output.status.success() => RemovalResult {
            success: true,
            message: format!("Removed {} ({})", dep, location.section),
        },
        Ok(output) => RemovalResult {
            success: false,
            message: format!(
                "Failed to remove {}: {}",
                dep,
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        },
        Err(e) => RemovalResult {
            success: false,
            message: format!("Failed to remove {dep}: {e}"),
        },
    }
}

// 定位依赖项
pub fn locate_dependency(
    dep: &str,
    cargo_data: &toml::Value,
) -> Result<DependencyLocation, DepCheckError> {
    let sections = [
        ("dependencies", None),
        ("dev-dependencies", Some("--dev")),
        ("build-dependencies", Some("--build")),
    ];

    for (section, flag) in sections {
        if let Some(table) = cargo_data.get(section) {
            if table.get(dep).is_some() {
                return Ok(DependencyLocation {
                    section: section.to_string(),
                    flag: flag.map(|s| s.to_string()),
                });
            }
        }
    }

    Err(DepCheckError::DependencyNotFound(dep.to_string()))
}

// 执行cargo udeps命令
pub fn execute_udeps() -> Result<String, DepCheckError> {
    let output = Command::new(UDEPS_CMD[0])
        .args(&UDEPS_CMD[1..])
        .output()
        .map_err(|e| DepCheckError::CommandFailed(e.to_string()))?;

    if output.status.code().unwrap_or(1) > 1 {
        return Err(DepCheckError::CommandFailed(format!(
            "Command failed (code {}):\n{}",
            output.status.code().unwrap_or(1),
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    ))
}

// 用户确认
pub fn get_confirmation(prompt: &str) -> bool {
    println!("{prompt} (y/n)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("y")
}

// 批量处理依赖项移除
pub fn process_removals(deps: &[String]) -> Vec<RemovalResult> {
    let cargo_content = match load_cargo_toml() {
        Ok(content) => content,
        Err(e) => {
            return vec![RemovalResult {
                success: false,
                message: format!("Failed to load Cargo.toml: {e}"),
            }];
        }
    };

    let cargo_data = match parse_cargo_toml(&cargo_content) {
        Ok(data) => data,
        Err(e) => {
            return vec![RemovalResult {
                success: false,
                message: format!("Failed to parse Cargo.toml: {e}"),
            }];
        }
    };

    let mut results = Vec::new();
    for dep in deps {
        match locate_dependency(dep, &cargo_data) {
            Ok(location) => results.push(remove_dependency(dep, &location)),
            Err(e) => results.push(RemovalResult {
                success: false,
                message: format!("Failed to locate dependency: {e}"),
            }),
        }
    }
    results
}

// 输出结果
pub fn print_results(results: &[RemovalResult]) {
    let successes: Vec<_> = results.iter().filter(|r| r.success).collect();
    let failures: Vec<_> = results.iter().filter(|r| !r.success).collect();

    if !successes.is_empty() {
        println!("\nSuccessfully removed:");
        for result in successes {
            println!("  {}", result.message);
        }
    }

    if !failures.is_empty() {
        println!("\nFailed to remove:");
        for result in failures {
            println!("  {}", result.message);
        }
    }
}

// 主流程
pub fn check_unused_dependencies() -> Result<(), DepCheckError> {
    if check_command("cargo-udeps").is_err() {
        return Err(DepCheckError::ToolMissing(
            "Please install cargo-udeps: cargo install cargo-udeps".into(),
        ));
    }

    println!("Scanning for unused dependencies...");
    let output = execute_udeps()?;
    let unused_deps = parse_udeps_output(&output);

    if unused_deps.is_empty() {
        println!("No unused dependencies found");
        return Ok(());
    }

    println!("\nFound unused dependencies:");
    println!("{}", unused_deps.join("\n"));

    if get_confirmation("\nConfirm removal of these dependencies?") {
        let results = process_removals(&unused_deps);
        print_results(&results);
    } else {
        println!("Operation cancelled");
    }

    Ok(())
}

// 解析udeps输出
pub fn parse_udeps_output(output: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let mut capturing = false;
    let dep_pattern = regex::Regex::new(r#""([^"]+)""#).unwrap();

    for line in output.lines().map(str::trim) {
        if line.is_empty() {
            capturing = false;
            continue;
        }
        if line.starts_with("unused dependencies:") {
            capturing = true;
            continue;
        }
        if capturing {
            if let Some(caps) = dep_pattern.captures(line) {
                deps.push(caps[1].to_string());
            }
        }
    }

    deps.sort();
    deps
}
