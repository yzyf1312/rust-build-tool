use std::error::Error;
use std::process::Command;

pub fn get_default_target() -> Result<String, Box<dyn Error>> {
    let output = Command::new("rustc").args(["-vV"]).output()?;

    if !output.status.success() {
        return Err("Failed to get rustc version".into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    for line in stdout.lines() {
        if line.starts_with("host: ") {
            return Ok(line.trim_start_matches("host: ").to_string());
        }
    }

    Err("Unable to determine default target platform".into())
}
