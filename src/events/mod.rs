use async_trait::async_trait;
use tokio::sync::broadcast;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum MinecraftEvent {
    BlockBreak {
        x: i32,
        y: i32,
        z: i32,
        block_type: String,
    },
    EntitySpawn {
        entity_id: i32,
        entity_type: String,
        x: f64,
        y: f64,
        z: f64,
    },
    PlayerJoin {
        player_id: String,
        player_name: String,
    },
    PlayerLeave {
        player_id: String,
    },
}

#[async_trait]
pub trait EventHandler: Send + Sync {
    async fn handle_event(&self, event: MinecraftEvent);
}

pub struct EventSystem {
    sender: broadcast::Sender<MinecraftEvent>,
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl EventSystem {
    pub fn new(buffer_size: usize) -> Self {
        let (sender, _) = broadcast::channel(buffer_size);
        Self {
            sender,
            handlers: Vec::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Arc<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<MinecraftEvent> {
        self.sender.subscribe()
    }

    pub async fn emit(&self, event: MinecraftEvent) {
        let _ = self.sender.send(event.clone());
        for handler in &self.handlers {
            handler.handle_event(event.clone()).await;
        }
    }
}