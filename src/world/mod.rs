use std::sync::Arc;
use tokio::sync::RwLock;
use crate::error::ModResult;

#[derive(Debug, Clone)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub block_type: String,
    pub metadata: Option<String>,
}

pub struct World {
    blocks: Arc<RwLock<Vec<Vec<Vec<Option<Block>>>>>>,
    size: (usize, usize, usize),
}

impl World {
    pub fn new(size_x: usize, size_y: usize, size_z: usize) -> Self {
        let blocks = vec![vec![vec![None; size_z]; size_y]; size_x];
        Self {
            blocks: Arc::new(RwLock::new(blocks)),
            size: (size_x, size_y, size_z),
        }
    }

    pub async fn set_block(&self, pos: BlockPosition, block: Block) -> ModResult<()> {
        let mut blocks = self.blocks.write().await;
        
        if pos.x >= 0 && (pos.x as usize) < self.size.0 &&
           pos.y >= 0 && (pos.y as usize) < self.size.1 &&
           pos.z >= 0 && (pos.z as usize) < self.size.2 {
            blocks[pos.x as usize][pos.y as usize][pos.z as usize] = Some(block);
            Ok(())
        } else {
            Err(ModError::World("Position out of bounds".into()))
        }
    }

    pub async fn get_block(&self, pos: &BlockPosition) -> Option<Block> {
        let blocks = self.blocks.read().await;
        blocks.get(pos.x as usize)?.get(pos.y as usize)?.get(pos.z as usize)?.clone()
    }
}