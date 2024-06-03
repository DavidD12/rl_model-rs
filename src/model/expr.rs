use super::*;
use crate::parser::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    True,
    False,
    ResourceEq(Reference<ResourceId>, Reference<StateId>),
    ResourceNe(Reference<ResourceId>, Reference<StateId>),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Implies(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        match self {
            Expr::True => Ok(()),
            Expr::False => Ok(()),
            Expr::ResourceEq(resource, _) => match resource {
                Reference::Unresolved(name, pos) => match map.get(name) {
                    Some(id) => {
                        *resource = Reference::Resolved(*id);
                        Ok(())
                    }
                    None => Err(RlError::Resolve {
                        element: format!("resource '{}'", name),
                        position: pos.clone(),
                    }),
                },
                Reference::Resolved(_) => Ok(()),
            },
            Expr::ResourceNe(resource, _) => match resource {
                Reference::Unresolved(name, pos) => match map.get(name) {
                    Some(id) => {
                        *resource = Reference::Resolved(*id);
                        Ok(())
                    }
                    None => Err(RlError::Resolve {
                        element: format!("resource '{}'", name),
                        position: pos.clone(),
                    }),
                },
                Reference::Resolved(_) => Ok(()),
            },
            Expr::Not(e) => e.resolve_resource(map),
            Expr::And(l, r) => {
                l.resolve_resource(map)?;
                r.resolve_resource(map)
            }
            Expr::Or(l, r) => {
                l.resolve_resource(map)?;
                r.resolve_resource(map)
            }
            Expr::Implies(l, r) => {
                l.resolve_resource(map)?;
                r.resolve_resource(map)
            }
        }
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        match self {
            Expr::True => Ok(()),
            Expr::False => Ok(()),
            Expr::ResourceEq(_, state) => match state {
                Reference::Unresolved(name, pos) => match map.get(name) {
                    Some(id) => {
                        *state = Reference::Resolved(*id);
                        Ok(())
                    }
                    None => Err(RlError::Resolve {
                        element: format!("state '{}'", name),
                        position: pos.clone(),
                    }),
                },
                Reference::Resolved(_) => Ok(()),
            },
            Expr::ResourceNe(_, state) => match state {
                Reference::Unresolved(name, pos) => match map.get(name) {
                    Some(id) => {
                        *state = Reference::Resolved(*id);
                        Ok(())
                    }
                    None => Err(RlError::Resolve {
                        element: format!("state '{}'", name),
                        position: pos.clone(),
                    }),
                },
                Reference::Resolved(_) => Ok(()),
            },
            Expr::Not(e) => e.resolve_state(map),
            Expr::And(l, r) => {
                l.resolve_state(map)?;
                r.resolve_state(map)
            }
            Expr::Or(l, r) => {
                l.resolve_state(map)?;
                r.resolve_state(map)
            }
            Expr::Implies(l, r) => {
                l.resolve_state(map)?;
                r.resolve_state(map)
            }
        }
    }
}

impl ToLang for Expr {
    fn to_lang(&self, skillset: &Skillset) -> String {
        match self {
            Expr::True => String::from("true"),
            Expr::False => String::from("false"),
            Expr::ResourceEq(resource, state) => {
                format!(
                    "{} == {}",
                    resource.to_lang(skillset),
                    state.to_lang(skillset)
                )
            }
            Expr::ResourceNe(resource, state) => {
                format!(
                    "{} != {}",
                    resource.to_lang(skillset),
                    state.to_lang(skillset)
                )
            }
            Expr::Not(e) => format!("(not {})", e.to_lang(skillset)),
            Expr::And(l, r) => format!("({} and {})", l.to_lang(skillset), r.to_lang(skillset)),
            Expr::Or(l, r) => format!("({} or {})", l.to_lang(skillset), r.to_lang(skillset)),
            Expr::Implies(l, r) => format!("({} => {})", l.to_lang(skillset), r.to_lang(skillset)),
        }
    }
}
