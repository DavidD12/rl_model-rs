pub mod error;
pub use error::*;

pub mod position;
pub use position::*;

use crate::model::*;

use std::fs;

lalrpop_mod!(grammar, "/parser/grammar.rs");

use line_col::LineColLookup;

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub position: Position,
}

impl Identifier {
    pub fn new(lookup: &LineColLookup, name: &str, offset: usize) -> Self {
        let name = name.into();
        let position = Position::new(lookup, offset);
        Self { name, position }
    }
}

pub fn parse_model_file(model: &mut Model, filename: &str) -> Result<(), RlError> {
    match fs::read_to_string(filename) {
        Ok(input) => {
            parse_model(model, &input)?;
            Ok(())
        }
        Err(e) => {
            let e = RlError::File {
                filename: filename.into(),
                message: format!("{:?}", e),
            };
            Err(e)
        }
    }
}

pub fn parse_model(model: &mut Model, input: &str) -> Result<(), RlError> {
    let lookup = LineColLookup::new(input);
    match grammar::ModelParser::new().parse(&lookup, model, input) {
        Ok(_) => Ok(()),
        Err(e) => Err(RlError::new_parse(&lookup, e)),
    }
}
