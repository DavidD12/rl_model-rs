use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

pub trait TerminateId: Id {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SuccessId(pub SkillId, pub usize);
impl Id for SuccessId {
    fn empty() -> Self {
        Self(SkillId::empty(), 0)
    }
}
impl TerminateId for SuccessId {}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct FailureId(pub SkillId, pub usize);
impl Id for FailureId {
    fn empty() -> Self {
        Self(SkillId::empty(), 0)
    }
}
impl TerminateId for FailureId {}

pub type Success = Terminate<SuccessId>;
pub type Failure = Terminate<FailureId>;

pub struct Terminate<I: TerminateId> {
    id: I,
    name: String,
    effects: Vec<Effect>,
    postcondition: Option<Expr>,
    position: Option<Position>,
}

impl<I: TerminateId> Terminate<I> {
    pub fn empty<S: Into<String>>(
        name: S,
        effects: Vec<Effect>,
        postcondition: Option<Expr>,
        position: Option<Position>,
    ) -> Self {
        let id = I::empty();
        let name = name.into();
        Self {
            id,
            name,
            effects,
            postcondition,
            position,
        }
    }

    pub fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    pub fn postcondition(&self) -> &Option<Expr> {
        &self.postcondition
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
        self.position
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
        self.position
    }
}
impl<I: TerminateId> ToLang for Terminate<I> {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = format!("{} {{\n", self.name);
        // Effects
        if !self.effects.is_empty() {
            s.push_str("\t\t\t\t\teffect {\n");
            for x in self.effects.iter() {
                s.push_str(&format!("\t\t\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t\t\t}\n");
        }
        // guard
        if let Some(post) = &self.postcondition {
            s.push_str(&format!(
                "\t\t\t\t\tpostcondition {}\n",
                post.to_lang(model)
            ));
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
