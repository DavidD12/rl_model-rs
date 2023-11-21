use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Parameter<I: Id> {
    id: I,
    variable: Variable,
}

impl<I: Id> Parameter<I> {
    pub fn new(variable: Variable) -> Self {
        let id = I::default();
        Self { id, variable }
    }

    pub fn variable(&self) -> &Variable {
        &self.variable
    }

    //---------- Resolve ----------
    pub fn resolve_type(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        self.variable.resolve_type(map)
    }
}

impl<I: Id> Named<I> for Parameter<I> {
    fn id(&self) -> I {
        self.id
    }

    fn set_id(&mut self, id: I) {
        self.id = id;
    }

    fn name(&self) -> &str {
        self.variable.name()
    }

    fn position(&self) -> Option<Position> {
        self.variable.position()
    }
}

impl<I: Id> ToLang for Parameter<I> {
    fn to_lang(&self, model: &Model) -> String {
        self.variable.to_lang(model)
    }
}

impl<I: Id> std::fmt::Display for Parameter<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

//------------------------- Skillset -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SkillsetParameterId(pub SkillsetId, pub usize);
impl Id for SkillsetParameterId {
    fn default() -> Self {
        Self(SkillsetId::default(), 0)
    }
}

pub type SkillsetParameter = Parameter<SkillsetParameterId>;

//------------------------- Skill -------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct SkillParameterId(pub SkillId, pub usize);
impl Id for SkillParameterId {
    fn default() -> Self {
        Self(SkillId::default(), 0)
    }
}

pub type SkillParameter = Parameter<SkillParameterId>;
