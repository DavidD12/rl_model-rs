use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Interrupt {
    interrupting: bool,
    effects: Vec<Effect>,
    postcondition: Option<Expr>,
    position: Option<Position>,
}

impl Interrupt {
    pub fn new(
        interrupting: bool,
        effects: Vec<Effect>,
        postcondition: Option<Expr>,
        position: Option<Position>,
    ) -> Self {
        Self {
            interrupting,
            effects,
            postcondition,
            position,
        }
    }

    pub fn interrupting(&self) -> bool {
        self.interrupting
    }

    pub fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn postcondition(&self) -> &Option<Expr> {
        &self.postcondition
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }

    //---------- Resolve ----------

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        for x in self.effects.iter_mut() {
            x.resolve_resource(map)?;
        }
        if let Some(post) = &mut self.postcondition {
            post.resolve_resource(map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        for x in self.effects.iter_mut() {
            x.resolve_state(map)?;
        }
        if let Some(post) = &mut self.postcondition {
            post.resolve_state(map)?;
        }
        Ok(())
    }
}

impl ToLang for Interrupt {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = String::from("\t\t\tinterrupt {\n");
        // Interrupting
        s.push_str(&format!("\t\t\t\tinterrupting {}\n", self.interrupting));
        // Effects
        if !self.effects.is_empty() {
            s.push_str("\t\t\t\teffect {\n");
            for x in self.effects.iter() {
                s.push_str(&format!("\t\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t\t}\n");
        }
        // Postcondition
        if let Some(post) = self.postcondition() {
            s.push_str(&format!("\t\t\t\tpostcondition {}\n", post.to_lang(model)));
        }
        //
        s.push_str("\t\t\t}\n");
        s
    }
}

impl std::fmt::Display for Interrupt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "interrupt")
    }
}
