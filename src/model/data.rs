use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct DataId(pub SkillsetId, pub usize);
impl Id for DataId {
    fn default() -> Self {
        Self(SkillsetId::default(), 0)
    }
}

#[derive(Debug)]
pub struct Data {
    id: DataId,
    name: String,
    rl_type: Reference<TypeId>,
    period: Option<Duration>,
    position: Option<Position>,
}

impl Data {
    pub fn new<S: Into<String>>(
        name: S,
        rl_type: Reference<TypeId>,
        period: Option<Duration>,
        position: Option<Position>,
    ) -> Self {
        let id = DataId::default();
        let name = name.into();
        Self {
            id,
            name,
            rl_type,
            period,
            position,
        }
    }

    pub fn rl_type(&self) -> &Reference<TypeId> {
        &self.rl_type
    }

    pub fn set_type(&mut self, id: TypeId) {
        self.rl_type = Reference::Resolved(id);
    }

    pub fn period(&self) -> Option<Duration> {
        self.period
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

impl Named<DataId> for Data {
    fn id(&self) -> DataId {
        self.id
    }

    fn set_id(&mut self, id: DataId) {
        self.id = id;
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl ToLang for Data {
    fn to_lang(&self, model: &Model) -> String {
        match self.period {
            Some(period) => format!(
                "{}: {} period {} ms\n",
                self.name,
                self.rl_type.to_lang(model),
                period.as_millis()
            ),
            None => format!("{}: {}\n", self.name, self.rl_type.to_lang(model)),
        }
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
