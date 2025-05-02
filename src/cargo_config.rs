use std::error::Error;
use std::fs;

const RELEASE_PROFILE_SETTINGS: [(&str, &str); 5] = [
    ("opt-level", "\'z\'"),
    ("lto", "true"),
    ("codegen-units", "1"),
    ("panic", "\'abort\'"),
    ("strip", "true"),
];

pub struct CargoConfigManager {
    cargo_toml: String,
    original_content: String,
}

impl CargoConfigManager {
    pub fn new(cargo_toml: &str) -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(cargo_toml)?;
        Ok(Self {
            cargo_toml: cargo_toml.to_string(),
            original_content: content.clone(),
        })
    }

    pub fn ensure_release_profile(&mut self) -> Result<(), Box<dyn Error>> {
        let mut lines: Vec<String> = self
            .original_content
            .lines()
            .map(|s| s.to_string())
            .collect();
        self.add_missing_section(&mut lines)?;
        self.update_profile_settings(&mut lines)?;
        self.write_file(&lines)?;
        Ok(())
    }

    fn add_missing_section(&self, lines: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
        if !self.original_content.contains("[profile.release]") {
            lines.push("[profile.release]".to_string());
        }
        Ok(())
    }

    fn update_profile_settings(&self, lines: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
        let section_start = self.find_section_index(lines)?;
        let existing_keys = self.parse_existing_settings(lines, section_start)?;

        for (key, value) in RELEASE_PROFILE_SETTINGS.iter() {
            if !existing_keys.contains(&key.to_string()) {
                let insert_pos = self.find_insert_position(lines, section_start)?;
                lines.insert(insert_pos, format!("{} = {}", key, value));
            }
        }

        Ok(())
    }

    fn find_section_index(&self, lines: &[String]) -> Result<usize, Box<dyn Error>> {
        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "[profile.release]" {
                return Ok(i);
            }
        }
        Ok(lines.len() - 1)
    }

    fn parse_existing_settings(
        &self,
        lines: &[String],
        start: usize,
    ) -> Result<Vec<String>, Box<dyn Error>> {
        let mut existing = Vec::new();
        for line in lines.iter().skip(start + 1) {
            if line.trim().is_empty() || line.trim().starts_with('[') {
                break;
            }
            if let Some(key) = line.split('=').next() {
                existing.push(key.trim().to_string());
            }
        }
        Ok(existing)
    }

    fn find_insert_position(
        &self,
        lines: &[String],
        start: usize,
    ) -> Result<usize, Box<dyn Error>> {
        for (i, line) in lines.iter().enumerate().skip(start + 1) {
            if line.trim().is_empty() || line.trim().starts_with('[') {
                return Ok(i);
            }
        }
        Ok(lines.len())
    }

    fn write_file(&self, lines: &[String]) -> Result<(), Box<dyn Error>> {
        fs::write(&self.cargo_toml, lines.join("\n"))?;
        Ok(())
    }

    pub fn restore(&self) -> Result<(), Box<dyn Error>> {
        fs::write(&self.cargo_toml, &self.original_content)?;
        Ok(())
    }
}
