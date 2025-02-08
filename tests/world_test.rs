#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_world_block_operations() {
        let world = World::new(10, 10, 10);
        
        let pos = BlockPosition { x: 5, y: 5, z: 5 };
        let block = Block {
            block_type: "stone".to_string(),
            metadata: None,
        };
        
        world.set_block(pos.clone(), block.clone()).await.unwrap();
        let retrieved_block = world.get_block(&pos).await.unwrap();
        
        assert_eq!(retrieved_block.block_type, block.block_type);
    }
}