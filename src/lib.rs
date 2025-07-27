mod config;

pub use crate::config::{CliError, load_blocks};
use std::path::Path;
use std::sync::OnceLock;

static BLOCK_DICTIONARY: OnceLock<Result<Vec<Block>, CliError>> = OnceLock::new();

/// Initializes the global block dictionary.
/// Returns an error if the dictionary has already been initialized.
pub fn initialize_block_dictionary<P: AsRef<Path>>(path: P) -> Result<(), CliError> {
    BLOCK_DICTIONARY.set(load_blocks(path)).map_err(|_| {
        CliError::IoError(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Block dictionary already initialized",
        ))
    })?;

    Ok(())
}

pub fn get_block_definition(value: usize) -> &'static Block {
    match BLOCK_DICTIONARY.get() {
        Some(Ok(dictionary)) => dictionary.get(value).unwrap_or(&Block::MISSING),
        Some(Err(e)) => {
            eprintln!("Error loading block dictionary: {}", e);
            &Block::MISSING
        }
        None => {
            eprintln!("Need to initialize block dictionary");
            &Block::MISSING
        }
    }
}

macro_rules! impl_getter {
    ($name:ident, $mask:expr) => {
        pub const fn $name(&self) -> bool {
            self.data & $mask != 0
        }
    };
}

/// Struct that stores generic block info.
/// Intended to be used for dictionaries, not individual blocks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Block {
    data: u8,
}

impl Block {
    const HOVERABLE_MASK: u8 = 1;
    const VISIBLE_MASK: u8 = 1 << 1;
    const BREAKABLE_MASK: u8 = 1 << 2;
    const COLLIDABLE_MASK: u8 = 1 << 3;
    const REPLACEABLE_MASK: u8 = 1 << 4;

    /// Represents the default block if any are missing or config fails to load.
    pub const MISSING: Block = Self::new(false, true, false, true, false);

    /// Creates a new block given all characteristics of it.
    pub(crate) const fn new(
        is_hoverable: bool,
        is_visible: bool,
        is_breakable: bool,
        is_collidable: bool,
        is_replaceable: bool,
    ) -> Self {
        let data: u8 = ((is_hoverable as u8) * Self::HOVERABLE_MASK)
            | ((is_visible as u8) * Self::VISIBLE_MASK)
            | ((is_breakable as u8) * Self::BREAKABLE_MASK)
            | ((is_collidable as u8) * Self::COLLIDABLE_MASK)
            | ((is_replaceable as u8) * Self::REPLACEABLE_MASK);

        Self { data }
    }

    impl_getter!(is_hoverable, Self::HOVERABLE_MASK);
    impl_getter!(is_visible, Self::VISIBLE_MASK);
    impl_getter!(is_breakable, Self::BREAKABLE_MASK);
    impl_getter!(is_collidable, Self::COLLIDABLE_MASK);
    impl_getter!(is_replaceable, Self::REPLACEABLE_MASK);
}

impl Default for Block {
    fn default() -> Self {
        Block::MISSING
    }
}
