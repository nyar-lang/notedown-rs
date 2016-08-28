mod config;
mod error;
mod parser;
pub mod utils;
// pub mod utils;
pub use config::ParserConfig;
pub use error::{Error, Result};

pub use notedown_ast::{
    nodes::{CommandKind, SmartLink},
    ASTKind, ASTNode,
};
