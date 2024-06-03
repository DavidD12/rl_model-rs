use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

pub trait TerminateId: Id {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct SuccessId(pub SkillId, pub usize);
impl Id for SuccessId {
    fn index(&self) -> usize {
        self.1
    }
}
impl SuccessId {
    pub fn skill(&self) -> SkillId {
        self.0
    }
}
impl TerminateId for SuccessId {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct FailureId(pub SkillId, pub usize);
impl Id for FailureId {
    fn index(&self) -> usize {
        self.1
    }
}
impl FailureId {
    pub fn skill(&self) -> SkillId {
        self.0
    }
}
impl TerminateId for FailureId {}

pub type Success = Terminate<SuccessId>;
pub type Failure = Terminate<FailureId>;

#[derive(Debug, Clone)]
pub struct Terminate<I: TerminateId> {
    id: I,
    name: String,
    postconditions: Vec<Postcondition>,
    effects: Vec<Effect>,
    position: Option<Position>,
}

impl<I: TerminateId> Terminate<I> {
    pub fn new<S: Into<String>>(
        name: S,
        postconditions: Vec<Postcondition>,
        effects: Vec<Effect>,
        position: Option<Position>,
    ) -> Self {
        let id = I::default();
        let name = name.into();
        Self {
            id,
            name,
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

impl Named<SuccessId> for Success {
    fn id(&self) -> SuccessId {
        self.id
    }
    fn set_id(&mut self, id: SuccessId) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}
impl Named<FailureId> for Failure {
    fn id(&self) -> FailureId {
        self.id
    }
    fn set_id(&mut self, id: FailureId) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}
impl<I: TerminateId> ToLang for Terminate<I> {
    fn to_lang(&self, skillset: &Skillset) -> String {
        let mut s = format!("{} {{\n", self.name);
        // Postcondition
        if !self.postconditions.is_empty() {
            s.push_str("\t\t\t\t\teffect {\n");
            for x in self.postconditions.iter() {
                s.push_str(&format!("\t\t\t\t\t\t{}\n", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t\t\t}\n");
        }
        // Effects
        if !self.effects.is_empty() {
            s.push_str("\t\t\t\t\teffect {\n");
            for x in self.effects.iter() {
                s.push_str(&format!("\t\t\t\t\t\t{}\n", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t\t\t}\n");
        }
        //
        s.push_str("\t\t\t\t}\n");
        s
    }
}

impl<I: TerminateId> std::fmt::Display for Terminate<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
