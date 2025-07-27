use block_dictionary::{Block, CliError, definition, initialize_block_dictionary, load_blocks};
use std::path::Path;

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

#[test]
fn test_block_dictionary() -> Result<(), CliError> {
    initialize_block_dictionary(Path::new("./tests/Blocks.toml"))?;

    let dirt = definition(2);

    assert!(dirt.is_breakable());
    assert!(dirt.is_visible());

    Ok(())
}
