use crate::parser::RlError;
use std::collections::HashMap;

use super::*;

#[derive(Debug, Clone)]
pub struct Transition {
    src: Reference<StateId>,
    dst: Reference<StateId>,
}

impl Transition {
    pub fn new(src: Reference<StateId>, dst: Reference<StateId>) -> Self {
        Self { src, dst }
    }

    pub fn src(&self) -> &Reference<StateId> {
        &self.src
    }

    pub fn dst(&self) -> &Reference<StateId> {
        &self.dst
    }
}

impl Transition {
    pub fn resolve(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        self.resolve_src(map)?;
        self.resolve_dst(map)
    }

    pub fn resolve_src(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        match &self.src {
            Reference::Unresolved(name, pos) => match map.get(name) {
                Some(id) => {
                    self.src = Reference::Resolved(*id);
                    Ok(())
                }
                None => Err(RlError::Resolve {
                    element: format!("state '{}'", name),
                    position: pos.clone(),
                }),
            },
            Reference::Resolved(_) => Ok(()),
        }
    }
    pub fn resolve_dst(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        match &self.dst {
            Reference::Unresolved(name, pos) => match map.get(name) {
                Some(id) => {
                    self.dst = Reference::Resolved(*id);
                    Ok(())
                }
                None => Err(RlError::Resolve {
                    element: format!("state '{}'", name),
                    position: pos.clone(),
                }),
            },
            Reference::Resolved(_) => Ok(()),
        }
    }
}

impl ToLang for Transition {
    fn to_lang(&self, skillset: &Skillset) -> String {
        format!(
            "{} -> {}",
            self.src.to_lang(skillset),
            self.dst.to_lang(skillset)
        )
    }
}

//-------------------------------------------------- Transitions --------------------------------------------------

#[derive(Debug, Clone)]
pub enum Transitions {
    All,
    List(Vec<Transition>),
}

impl Transitions {
    pub fn resolve(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        match self {
            Transitions::All => Ok(()),
            Transitions::List(l) => {
                for x in l {
                    x.resolve(map)?;
                }
                Ok(())
            }
        }
    }
}

impl ToLang for Transitions {
    fn to_lang(&self, skillset: &Skillset) -> String {
        match self {
            Transitions::All => format!("\t\t\ttransition all\n"),
            Transitions::List(l) => {
                let mut s = String::from("\t\t\ttransition {\n");
                for x in l {
                    s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(skillset)));
                }
                s.push_str("\t\t\t}\n");
                s
            }
        }
    }
}
