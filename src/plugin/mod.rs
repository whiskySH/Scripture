use async_trait::async_trait;
use dlopen::wrapper::{Container, WrapperApi};
use std::sync::Arc;
use crate::error::ModResult;
use crate::config::ModConfig;

#[async_trait]
pub trait Plugin: Send + Sync {
    async fn enable(&self) -> ModResult<()>;
    async fn disable(&self) -> ModResult<()>;
    fn get_config(&self) -> &ModConfig;
}

pub struct PluginManager {
    plugins: Vec<Arc<Container<Box<dyn Plugin>>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub async fn load_plugin(&mut self, path: &str) -> ModResult<()> {
        let container: Container<Box<dyn Plugin>> = unsafe {
            Container::load(path)
                .map_err(|e| ModError::Plugin(format!("Failed to load plugin: {}", e)))?
        };
        
        let plugin = Arc::new(container);
        plugin.enable().await?;
        self.plugins.push(plugin);
        Ok(())
    }

    pub async fn unload_all(&mut self) -> ModResult<()> {
        for plugin in self.plugins.drain(..) {
            plugin.disable().await?;
        }
        Ok(())
    }
}