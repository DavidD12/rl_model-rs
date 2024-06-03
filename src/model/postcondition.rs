use super::*;
use crate::parser::*;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct PostconditionId(pub usize);
impl Id for PostconditionId {
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Postcondition {
    id: PostconditionId,
    name: String,
    expr: Expr,
    position: Option<Position>,
}

impl Postcondition {
    pub fn new<S: Into<String>>(name: S, expr: Expr, position: Option<Position>) -> Self {
        let id = PostconditionId::default();
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

impl Named<PostconditionId> for Postcondition {
    fn id(&self) -> PostconditionId {
        self.id
    }

    fn set_id(&mut self, id: PostconditionId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl ToLang for Postcondition {
    fn to_lang(&self, skillset: &Skillset) -> String {
        format!("{}: {}", self.name, self.expr.to_lang(skillset))
    }
}

impl std::fmt::Display for Postcondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
