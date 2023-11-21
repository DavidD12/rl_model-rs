use super::*;
use crate::parser::Position;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct TypeId(pub usize);
impl Id for TypeId {
    fn empty() -> Self {
        Self(0)
    }
}

pub struct RlType {
    id: TypeId,
    name: String,
    position: Option<Position>,
}

impl RlType {
    pub fn empty<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = TypeId::empty();
        let name = name.into();
        Self { id, name, position }
    }
}

impl Named<TypeId> for RlType {
    fn id(&self) -> TypeId {
        self.id
    }
    fn set_id(&mut self, id: TypeId) {
        self.id = id;
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
        self.position
    }
}

impl std::fmt::Display for RlType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
