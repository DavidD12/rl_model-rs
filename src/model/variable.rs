use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Variable {
    name: String,
    rl_type: Reference<TypeId>,
    position: Option<Position>,
}

impl Variable {
    pub fn new<S: Into<String>>(
        name: S,
        rl_type: Reference<TypeId>,
        position: Option<Position>,
    ) -> Self {
        let name = name.into();
        Self {
            name,
            rl_type,
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn rl_type(&self) -> &Reference<TypeId> {
        &self.rl_type
    }

    pub fn set_type(&mut self, id: TypeId) {
        self.rl_type = Reference::Resolved(id);
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        match self.rl_type() {
            Reference::Unresolved(name, pos) => match map.get(name) {
                Some(id) => {
                    self.set_type(*id);
                    Ok(())
                }
                None => Err(RlError::Resolve {
                    element: format!("type '{}'", name),
                    position: pos.clone(),
                }),
            },
            Reference::Resolved(_) => Ok(()),
        }
    }
}

impl ToLang for Variable {
    fn to_lang(&self, model: &Model) -> String {
        format!("{}: {}", self.name, self.rl_type.to_lang(model))
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
