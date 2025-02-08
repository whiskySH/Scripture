#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_save_load() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("test_config.toml");
        
        let config = ModConfig {
            name: "test_mod".to_string(),
            version: "1.0.0".to_string(),
            description: Some("Test mod".to_string()),
            authors: vec!["Test Author".to_string()],
            dependencies: vec![],
        };
        
        config.save(&config_path).unwrap();
        let loaded_config = ModConfig::load(&config_path).unwrap();
        
        assert_eq!(config.name, loaded_config.name);
        assert_eq!(config.version, loaded_config.version);
        assert_eq!(config.description, loaded_config.description);
    }
}