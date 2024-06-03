use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Interrupt {
    effects: Vec<Effect>,
    postconditions: Vec<Postcondition>,
    position: Option<Position>,
}

impl Interrupt {
    pub fn new(
        postconditions: Vec<Postcondition>,
        effects: Vec<Effect>,
        position: Option<Position>,
    ) -> Self {
        Self {
            effects,
            postconditions,
            position,
        }
    }

    pub fn postconditions(&self) -> &Vec<Postcondition> {
        &self.postconditions
    }

    pub fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }

    //---------- Resolve ----------

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        for x in self.effects.iter_mut() {
            x.resolve_resource(map)?;
        }
        for x in self.postconditions.iter_mut() {
            x.resolve_resource(map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        for x in self.effects.iter_mut() {
            x.resolve_state(map)?;
        }
        for x in self.postconditions.iter_mut() {
            x.resolve_state(map)?;
        }
        Ok(())
    }
}

impl ToLang for Interrupt {
    fn to_lang(&self, skillset: &Skillset) -> String {
        let mut s = String::from("\t\t\tinterrupt {\n");
        // Postcondition
        if !self.postconditions.is_empty() {
            s.push_str("\t\t\t\tpostcondition {\n");
            for x in self.postconditions() {
                s.push_str(&format!("\t\t\t\t\t{}\n", x.to_lang(skillset)));
            }
            s.push_str("\t\t\t\t}\n");
        }
        // Effects
        if !self.effects.is_empty() {
            s.push_str("\t\t\t\teffect {\n");
            for x in self.effects.iter() {
                s.push_str(&format!("\t\t\t\t\t{}\n", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t\t}\n");
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
