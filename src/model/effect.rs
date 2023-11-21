use super::*;
use crate::parser::*;
use std::collections::HashMap;

pub struct Effect {
    resource: Reference<ResourceId>,
    state: Reference<StateId>,
}

impl Effect {
    pub fn new(resource: Reference<ResourceId>, state: Reference<StateId>) -> Self {
        Self { resource, state }
    }

    pub fn resource(&self) -> &Reference<ResourceId> {
        &self.resource
    }

    pub fn state(&self) -> &Reference<StateId> {
        &self.state
    }

    //---------- Resolve ----------

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        match &mut self.resource {
            Reference::Unresolved(name, pos) => match map.get(name) {
                Some(id) => {
                    self.resource = Reference::Resolved(*id);
                    Ok(())
                }
                None => Err(RlError::Resolve {
                    element: format!("resource '{}'", name),
                    position: *pos,
                }),
            },
            Reference::Resolved(_) => Ok(()),
        }
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        match &mut self.state {
            Reference::Unresolved(name, pos) => match map.get(name) {
                Some(id) => {
                    self.state = Reference::Resolved(*id);
                    Ok(())
                }
                None => Err(RlError::Resolve {
                    element: format!("resource '{}'", name),
                    position: *pos,
                }),
            },
            Reference::Resolved(_) => Ok(()),
        }
    }
}

impl ToLang for Effect {
    fn to_lang(&self, model: &Model) -> String {
        format!(
            "{} -> {}",
            self.resource.to_lang(model),
            self.state.to_lang(model)
        )
    }
}
