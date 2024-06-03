use super::*;
use crate::parser::Position;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Reference<I: Id> {
    Unresolved(String, Option<Position>),
    Resolved(I),
}

impl<I: Id> Reference<I> {
    pub fn resolved(&self) -> I {
        match self {
            Reference::Unresolved(_, _) => panic!("reference must be resolved"),
            Reference::Resolved(id) => *id,
        }
    }

    pub fn resolve(&self, map: &HashMap<String, I>, info: &str) -> Result<Self, RlError> {
        match self {
            Reference::Unresolved(name, pos) => match map.get(name) {
                Some(id) => Ok(Self::Resolved(*id)),
                None => Err(RlError::Resolve {
                    element: format!("{} '{}'", info, name),
                    position: pos.clone(),
                }),
            },
            Reference::Resolved(_) => Ok(self.clone()),
        }
    }
}

impl ToLang for Reference<TypeId> {
    fn to_lang(&self, skillset: &Skillset) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", skillset.get(*id).unwrap()),
        }
    }
}
impl ToLang for Reference<ResourceId> {
    fn to_lang(&self, skillset: &Skillset) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", skillset.get(*id).unwrap()),
        }
    }
}

impl ToLang for Reference<StateId> {
    fn to_lang(&self, skillset: &Skillset) -> String {
        match self {
            Reference::Unresolved(name, _) => format!("{}/* ? */", name),
            Reference::Resolved(id) => format!("{}", skillset.get(*id).unwrap()),
        }
    }
}

// impl ToLang for Reference<DataId> {
//     fn to_lang(&self, skillset: &crate::Skillset) -> String {
//         match self {
//             Reference::Unresolved(name, _) => format!("{}/* ? */", name),
//             Reference::Resolved(id) => format!("{}", skillset.get(*id).unwrap()),
//         }
//     }
// }

// impl ToLang for Reference<SkillsetId> {
//     fn to_lang(&self, skillset: &crate::Model) -> String {
//         match self {
//             Reference::Unresolved(name, _) => format!("{}/* ? */", name),
//             Reference::Resolved(id) => format!("{}", skillset.get(*id).unwrap()),
//         }
//     }
// }

// impl ToLang for Reference<EventId> {
//     fn to_lang(&self, skillset: &Model) -> String {
//         match self {
//             Reference::Unresolved(name, _) => format!("{}/* ? */", name),
//             Reference::Resolved(id) => format!("{}", skillset.get(*id).unwrap()),
//         }
//     }
// }
