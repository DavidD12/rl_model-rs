use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct Progress {
    period: Duration,
    message: Vec<Variable>,
    position: Option<Position>,
}

impl Progress {
    pub fn new(period: Duration, message: Vec<Variable>, position: Option<Position>) -> Self {
        Self {
            period,
            message,
            position,
        }
    }

    pub fn period(&self) -> Duration {
        self.period
    }

    pub fn message(&self) -> &Vec<Variable> {
        &self.message
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        for x in self.message.iter_mut() {
            x.resolve_type(map)?;
        }
        Ok(())
    }
}

impl ToLang for Progress {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = String::from("\t\t\tprogress {\n");
        // guard
        s.push_str(&format!("\t\t\t\tperiod {} ms\n", self.period.as_millis()));
        // Effects
        if !self.message.is_empty() {
            s.push_str("\t\t\t\tmessage {\n");
            for x in self.message.iter() {
                s.push_str(&format!("\t\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t\t}\n");
        }
        //
        s.push_str("\t\t\t}\n");
        s
    }
}

impl std::fmt::Display for Progress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "progress")
    }
}
