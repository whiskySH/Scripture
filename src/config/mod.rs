use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use crate::error::{ModError, ModResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModConfig {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub dependencies: Vec<String>,
}

impl ModConfig {
    pub fn load(path: &Path) -> ModResult<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| ModError::Config(format!("Failed to read config: {}", e)))?;
        
        toml::from_str(&content)
            .map_err(|e| ModError::Config(format!("Failed to parse config: {}", e)))
    }

    pub fn save(&self, path: &Path) -> ModResult<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| ModError::Config(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(path, content)
            .map_err(|e| ModError::Config(format!("Failed to write config: {}", e)))
    }
}
