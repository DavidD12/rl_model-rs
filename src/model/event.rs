use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct EventId(pub SkillsetId, pub usize);
impl Id for EventId {
    fn default() -> Self {
        Self(SkillsetId::default(), 0)
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    id: EventId,
    name: String,
    guard: Option<Expr>,
    effects: Vec<Effect>,
    position: Option<Position>,
}

impl Event {
    pub fn new<S: Into<String>>(
        name: S,
        guard: Option<Expr>,
        effects: Vec<Effect>,
        position: Option<Position>,
    ) -> Self {
        let id = EventId::default();
        let name = name.into();
        Self {
            id,
            name,
            guard,
            effects,
            position,
        }
    }

    pub fn guard(&self) -> &Option<Expr> {
        &self.guard
    }

    pub fn effects(&self) -> &Vec<Effect> {
        &self.effects
    }

    //---------- Resolve ----------

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        match &mut self.guard {
            Some(e) => {
                e.resolve_resource(map)?;
            }
            None => {}
        }
        for x in self.effects.iter_mut() {
            x.resolve_resource(map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        match &mut self.guard {
            Some(e) => {
                e.resolve_state(map)?;
            }
            None => {}
        }
        for x in self.effects.iter_mut() {
            x.resolve_state(map)?;
        }
        Ok(())
    }
}

impl Named<EventId> for Event {
    fn id(&self) -> EventId {
        self.id
    }

    fn set_id(&mut self, id: EventId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl ToLang for Event {
    fn to_lang(&self, model: &Model) -> String {
        let mut s = String::new();
        s.push_str(&format!("\t\t{} {{\n", self.name));
        // guard
        match &self.guard {
            Some(guard) => s.push_str(&format!("\t\t\tguard {}\n", guard.to_lang(model))),
            None => {}
        }
        // Effects
        if !self.effects.is_empty() {
            s.push_str("\t\t\teffect {\n");
            for x in self.effects.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(model)))
            }
            s.push_str("\t\t\t}\n");
        }
        //
        s.push_str("\t\t}\n");
        s
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
