use super::*;
use crate::parser::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct StateId(pub ResourceId, pub usize);
impl Id for StateId {
    fn empty() -> Self {
        Self(ResourceId::empty(), 0)
    }
}

pub struct State {
    id: StateId,
    name: String,
    position: Option<Position>,
}

impl State {
    pub fn empty<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = StateId::empty();
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
        self.position
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
