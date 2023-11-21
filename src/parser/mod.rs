pub mod parser;
pub use parser::*;

pub mod error;
pub use error::*;

pub mod position;
pub use position::*;

use crate::model::*;

use line_col::LineColLookup;

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub position: Position,
}

impl Identifier {
    pub fn new(file: &str, lookup: &LineColLookup, name: &str, offset: usize) -> Self {
        let name = name.into();
        let position = Position::new(file, lookup, offset);
        Self { name, position }
    }
}

pub fn parse_file(model: &mut Model, file: &str) -> Result<(), RlError> {
    let mut parser = Parser::new(model);
    parser.add(file);

    parser.parse()
}
