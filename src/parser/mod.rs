pub mod parser;
pub use parser::*;

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
    pub fn new(file: &str, lookup: &LineColLookup, name: &str, offset: usize) -> Self {
        let name = name.into();
        let position = Position::new(file, lookup, offset);
        Self { name, position }
    }
}

pub fn parse_file(model: &mut Model, file: &str) -> Result<(), RlError> {
    let mut parser = Parser::new(model);
    parser.add(file);

    loop {
        match parser.next() {
            None => return Ok(()),
            Some(file) => match fs::read_to_string(&file) {
                Ok(input) => {
                    let lookup = LineColLookup::new(&input);
                    match grammar::ModelParser::new().parse(&lookup, &mut parser, &input) {
                        Ok(_) => {}
                        Err(e) => return Err(RlError::new_parse(&file, &lookup, e)),
                    }
                }
                Err(e) => {
                    let e = RlError::File {
                        filename: file,
                        message: format!("{:?}", e),
                    };
                    return Err(e);
                }
            },
        }
    }
}
