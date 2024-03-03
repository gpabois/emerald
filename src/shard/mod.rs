mod ast;
mod value;

use std::{error::Error, str::FromStr};

pub use value::Value;

use ast::Ast;

use self::ast::walker::RefWalker;

/// A shard is a piece of data within a Jewel.
pub struct Shard {
    /// Shard's AST
    pub ast: Ast,
}

impl FromStr for Shard {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ast = ast::Ast::from_str(s).unwrap();
        Ok(Self { ast })
    }
}

pub struct Chapter {}

impl Shard {
    /// Read the shard from a stream.
    pub fn read<R: std::io::Read>(mut stream: R) -> Result<Self, Box<dyn Error>> {
        let mut doc = String::default();
        stream.read_to_string(&mut doc)?;
        Self::from_str(&doc)
    }

    /// Read the shard from a string.
    pub fn walk_ref(&self) -> RefWalker<'_> {
        self.ast.walk_ref()
    }
}
