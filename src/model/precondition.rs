use super::*;
use crate::parser::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PreconditionId(pub SkillId, pub usize);
impl Id for PreconditionId {
    fn default() -> Self {
        Self(SkillId::default(), 0)
    }
}

#[derive(Debug, Clone)]
pub struct Precondition {
    id: PreconditionId,
    name: String,
    expr: Expr,
    position: Option<Position>,
}

impl Precondition {
    pub fn new<S: Into<String>>(name: S, expr: Expr, position: Option<Position>) -> Self {
        let id = PreconditionId::default();
        let name = name.into();
        Self {
            id,
            name,
            expr,
            position,
        }
    }

    pub fn expr(&self) -> &Expr {
        &self.expr
    }

    //---------- Resolve ----------

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        self.expr.resolve_resource(map)
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        self.expr.resolve_state(map)
    }
}

impl Named<PreconditionId> for Precondition {
    fn id(&self) -> PreconditionId {
        self.id
    }

    fn set_id(&mut self, id: PreconditionId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl ToLang for Precondition {
    fn to_lang(&self, model: &Model) -> String {
        format!("{}: {}", self.name, self.expr.to_lang(model))
    }
}

impl std::fmt::Display for Precondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
