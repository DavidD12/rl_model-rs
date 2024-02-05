use crate::model::Model;
use crate::parser::error::*;
use line_col::LineColLookup;
use std::fs;

lalrpop_mod!(grammar, "/parser/grammar.rs");

pub struct Parser {
    current: Option<String>,
    todo: Vec<String>,
    done: Vec<String>,
    pub model: Model,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            current: Default::default(),
            todo: Default::default(),
            done: Default::default(),
            model: Default::default(),
        }
    }
}

impl Parser {
    pub fn new<S: Into<String>>(file: S) -> Self {
        Self {
            current: None,
            todo: vec![file.into()],
            done: vec![],
            model: Default::default(),
        }
    }

    pub fn current(&self) -> &Option<String> {
        &self.current
    }

    pub fn file(&self) -> &str {
        self.current.as_ref().unwrap()
    }

    pub fn todo(&self) -> &Vec<String> {
        &self.todo
    }

    pub fn done(&self) -> &Vec<String> {
        &self.done
    }

    //------------------------- -------------------------

    pub fn files(&self) -> Vec<String> {
        let mut v = vec![];
        if let Some(f) = &self.current {
            v.push(f.clone());
        }
        v.extend(self.todo.clone().into_iter());
        v.extend(self.done.clone().into_iter());
        v
    }

    pub fn add<S: Into<String>>(&mut self, file: S) -> bool {
        let file: String = file.into();
        if self.files().contains(&file) {
            false
        } else {
            self.todo.push(file);
            true
        }
    }

    pub fn next(&mut self) -> Option<String> {
        if let Some(file) = &self.current {
            self.done.push(file.clone());
        }
        match self.todo.pop() {
            None => None,
            Some(file) => {
                self.current = Some(file.clone());
                Some(file)
            }
        }
    }

    pub fn parse(&mut self) -> Result<(), RlError> {
        loop {
            match self.next() {
                None => return Ok(()),
                Some(file) => match fs::read_to_string(&file) {
                    Ok(input) => {
                        let lookup = LineColLookup::new(&input);
                        match grammar::ModelParser::new().parse(&lookup, self, &input) {
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
}
