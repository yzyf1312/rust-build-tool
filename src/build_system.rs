use std::env::consts::EXE_SUFFIX;
use std::error::Error;
use std::path::PathBuf;
use std::process::Command;

use crate::cargo_config::CargoConfigManager;
use crate::dependency_checker;

pub struct BuildSystem {
    target: String,
    use_upx: bool,
    clean: bool,
    executable: PathBuf,
}

impl BuildSystem {
    pub fn new(target: &str, use_upx: bool, clean: bool) -> Result<Self, Box<dyn Error>> {
        // Check dependencies
        dependency_checker::check_command("cargo")?;
        dependency_checker::check_rust_nightly()?;
        if use_upx {
            dependency_checker::check_upx_lzma()?;
        }

        // Get project name
        let project_name = Self::parse_project_name()?;

        // Get target directory
        let target_dir = Self::get_target_directory()?;

        // Get executable path
        let executable = Self::get_executable_path(&target_dir, &project_name, target)?;

        Ok(Self {
            target: target.to_string(),
            use_upx,
            clean,
            executable,
        })
    }

    fn parse_project_name() -> Result<String, Box<dyn Error>> {
        let cargo_toml = std::fs::read_to_string("Cargo.toml")?;
        let name_line = cargo_toml
            .lines()
            .find(|line| line.trim().starts_with("name = "))
            .ok_or("Failed to find project name in Cargo.toml")?;

        let name = name_line
            .split('=')
            .nth(1)
            .ok_or("Invalid name format in Cargo.toml")?
            .trim()
            .trim_matches('"')
            .to_string();

        Ok(name)
    }

    fn get_target_directory() -> Result<PathBuf, Box<dyn Error>> {
        let output = Command::new("cargo")
            .args(["metadata", "--format-version=1", "--no-deps"])
            .output()?;

        let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)?;
        let target_dir = metadata["target_directory"]
            .as_str()
            .ok_or("Failed to get target directory")?;

        Ok(PathBuf::from(target_dir))
    }

    fn get_executable_path(
        target_dir: &PathBuf,
        project_name: &str,
        target: &str,
    ) -> Result<PathBuf, Box<dyn Error>> {
        let platform_suffix = if target.contains("uefi") {
            ".efi"
        } else {
            EXE_SUFFIX
        };

        Ok(target_dir
            .join(target)
            .join("release")
            .join(format!("{}{}", project_name, platform_suffix)))
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        // Modify and restore Cargo config
        let mut config = CargoConfigManager::new("Cargo.toml")?;
        config.ensure_release_profile()?;

        if self.clean {
            self.clean()?;
        }

        self.build()?;

        if self.use_upx {
            self.compress()?;
        }

        self.show_result()?;

        config.restore()?;
        Ok(())
    }

    fn clean(&self) -> Result<(), Box<dyn Error>> {
        println!("Cleaning previous build files...");
        Command::new("cargo").args(["clean"]).status()?;
        Ok(())
    }

    fn build(&self) -> Result<(), Box<dyn Error>> {
        println!("Building optimized executable...");
        println!("Target: {}", self.target);
        Command::new("cargo")
            .args([
                "+nightly",
                "build",
                "-Z",
                "build-std=std,panic_abort",
                "-Z",
                "build-std-features=panic_immediate_abort",
                "--target",
                &self.target,
                "--release",
            ])
            .status()?;
        println!("Build complete!");
        Ok(())
    }

    fn compress(&self) -> Result<(), Box<dyn Error>> {
        println!("Compressing with UPX: {}", self.executable.display());
        Command::new("upx")
            .args(["--best", "--lzma", self.executable.to_str().unwrap()])
            .status()?;
        Ok(())
    }

    fn show_result(&self) -> Result<(), Box<dyn Error>> {
        let size_kb = self.executable.metadata()?.len() as f64 / 1024.0;
        println!("\nBuild complete! Final size: {:.1} KB", size_kb);
        println!("Executable path: {}", self.executable.display());
        Ok(())
    }
}
