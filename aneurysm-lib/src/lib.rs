pub mod interpreter;
pub(crate) mod modular;

/// The default filename to use in case one isn't specified by the user
pub const DEFAULT_FILENAME: &str = "main.bf";

/// The default cell size to use in case one isn't specified by the user
pub const DEFAULT_CELL_SIZE: usize = 30000;
