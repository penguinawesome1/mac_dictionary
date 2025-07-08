use block_dictionary::{ Block, load_blocks, CliError };

#[test]
fn test_config() -> Result<(), CliError> {
    let blocks: Vec<Block> = load_blocks("./tests/Blocks.toml")?;
    let air: Block = blocks[0];
    let bedrock: Block = blocks[4];

    assert!(air.is_replaceable());
    assert!(!bedrock.is_breakable());
    assert!(bedrock.is_visible());

    Ok(())
}

#[test]
fn test_missing() {
    assert!(Block::MISSING.is_visible());
    assert!(!Block::MISSING.is_replaceable());
    assert_eq!(Block::MISSING, Block::default());
}
