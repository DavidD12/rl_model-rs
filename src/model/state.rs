use super::*;
use crate::parser::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct StateId(pub ResourceId, pub usize);
impl Id for StateId {
    fn default() -> Self {
        Self(ResourceId::default(), 0)
    }
}

#[derive(Debug)]
pub struct State {
    id: StateId,
    name: String,
    position: Option<Position>,
}

impl State {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = StateId::default();
        let name = name.into();
        Self { id, name, position }
    }
}

impl Named<StateId> for State {
    fn id(&self) -> StateId {
        self.id
    }
    fn set_id(&mut self, id: StateId) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
