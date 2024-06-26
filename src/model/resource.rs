use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct ResourceId(pub usize);
impl Id for ResourceId {
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Resource {
    id: ResourceId,
    name: String,
    states: Vec<State>,
    initial: Reference<StateId>,
    transitions: Transitions,
    position: Option<Position>,
}

impl Resource {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = ResourceId::default();
        let name = name.into();
        Self {
            id,
            name,
            states: Vec::new(),
            initial: Reference::Unresolved("?".into(), None),
            transitions: Transitions::All,
            position,
        }
    }

    //---------- State ----------

    pub fn states(&self) -> &Vec<State> {
        &self.states
    }

    pub fn add_state(&mut self, mut state: State) -> StateId {
        let id = StateId(self.id, self.states.len());
        state.set_id(id);
        self.states.push(state);
        id
    }

    pub fn get_state(&self, id: StateId) -> Option<&State> {
        let StateId(resource_id, state_id) = id;
        if self.id != resource_id {
            None
        } else {
            self.states.get(state_id)
        }
    }

    pub fn initial(&self) -> StateId {
        self.initial.resolved()
    }

    pub fn set_initial(&mut self, state: Reference<StateId>) {
        self.initial = state;
    }

    pub fn state_map(&self) -> HashMap<String, StateId> {
        let mut map = HashMap::new();
        for x in self.states.iter() {
            map.insert(x.name().into(), x.id());
        }
        map
    }

    pub fn get_state_from_name(&self, name: &str) -> Option<&State> {
        for x in self.states.iter() {
            if x.name() == name {
                return Some(x);
            }
        }
        None
    }

    //---------- Transition ----------

    pub fn transitions(&self) -> &Transitions {
        &self.transitions
    }

    pub fn set_transitions(&mut self, transitions: Transitions) {
        self.transitions = transitions;
    }

    pub fn transitions_list(&self) -> Vec<Transition> {
        match self.transitions() {
            Transitions::All => {
                let mut transitions = vec![];
                for src in self.states.iter() {
                    let r_src = Reference::Resolved(src.id());
                    for dst in self.states.iter() {
                        let r_dst = Reference::Resolved(dst.id());
                        if src.id() != dst.id() {
                            transitions.push(Transition::new(r_src.clone(), r_dst.clone()));
                        }
                    }
                }
                transitions
            }
            Transitions::List(transitions) => transitions.clone(),
        }
    }

    //---------- Duplicate ----------

    pub fn names(&self) -> Vec<(String, Option<Position>)> {
        let mut v = Vec::new();
        for x in self.states.iter() {
            v.push((x.name().into(), x.position()))
        }
        v
    }

    //---------- Resolve ----------

    pub fn resolve_state(&mut self) -> Result<(), RlError> {
        let map = self.state_map();
        self.resolve_initial_state(&map)?;
        self.transitions.resolve(&map)?;
        Ok(())
    }

    fn resolve_initial_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        match &self.initial {
            Reference::Unresolved(name, pos) => match map.get(name) {
                Some(id) => {
                    self.initial = Reference::Resolved(*id);
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

impl Named<ResourceId> for Resource {
    fn id(&self) -> ResourceId {
        self.id
    }

    fn set_id(&mut self, id: ResourceId) {
        self.id = id;
        for state in self.states.iter_mut() {
            let StateId(_, index) = state.id();
            state.set_id(StateId(id, index));
        }
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<StateId, State> for Resource {
    fn get(&self, id: StateId) -> Option<&State> {
        self.get_state(id)
    }
}

//------------------------- ToLang -------------------------

impl ToLang for Resource {
    fn to_lang(&self, skillset: &Skillset) -> String {
        let mut s = String::new();
        s.push_str(&format!("\t\t{} {{\n", self.name));
        // state
        s.push_str("\t\t\tstate {");
        for x in self.states.iter() {
            s.push_str(&format!(" {}", x));
        }
        s.push_str(" }\n");
        // initial
        s.push_str(&format!(
            "\t\t\tinitial {}\n",
            self.initial.to_lang(skillset)
        ));
        // transitions
        s.push_str(&self.transitions.to_lang(skillset));
        //
        s.push_str("\t\t}\n");
        s
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
