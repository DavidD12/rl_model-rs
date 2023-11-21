use std::collections::HashMap;

use super::*;
use crate::parser::RlError;

#[derive(Debug, Clone)]
pub struct Model {
    types: Vec<RlType>,
    skillsets: Vec<Skillset>,
}

impl Model {
    //---------- Type ----------

    pub fn types(&self) -> &Vec<RlType> {
        &self.types
    }

    pub fn add_type(&mut self, mut rl_type: RlType) -> TypeId {
        let id = TypeId(self.types.len());
        rl_type.set_id(id);
        self.types.push(rl_type);
        id
    }

    pub fn get_type(&self, id: TypeId) -> Option<&RlType> {
        let TypeId(index) = id;
        self.types.get(index)
    }

    pub fn type_map(&self) -> HashMap<String, TypeId> {
        let mut map = HashMap::new();
        for x in self.types.iter() {
            map.insert(x.name().into(), x.id());
        }
        map
    }

    //---------- Skillset ----------

    pub fn skillsets(&self) -> &Vec<Skillset> {
        &self.skillsets
    }

    pub fn add_skillset(&mut self, mut skillset: Skillset) -> SkillsetId {
        let id = SkillsetId(self.skillsets.len());
        skillset.set_id(id);
        self.skillsets.push(skillset);
        id
    }

    pub fn get_skillset(&self, id: SkillsetId) -> Option<&Skillset> {
        let SkillsetId(index) = id;
        self.skillsets.get(index)
    }

    pub fn get_skillset_mut(&mut self, id: SkillsetId) -> Option<&mut Skillset> {
        let SkillsetId(index) = id;
        self.skillsets.get_mut(index)
    }

    //---------- Duplicate ----------

    pub fn type_naming(&self) -> Vec<Naming> {
        self.types.iter().map(|x| x.naming()).collect()
    }
    pub fn skillset_naming(&self) -> Vec<Naming> {
        self.skillsets.iter().map(|x| x.naming()).collect()
    }

    pub fn duplicate(&self) -> Result<(), RlError> {
        // Types
        check_duplicate(self.type_naming())?;
        // Skillset
        check_duplicate(self.skillset_naming())?;

        // skillset
        for x in self.skillsets.iter() {
            x.duplicate(self)?;
        }
        //
        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve(&mut self) -> Result<(), RlError> {
        self.resolve_type()?;
        self.resolve_resource()?;
        self.resolve_state()
    }

    fn resolve_type(&mut self) -> Result<(), RlError> {
        let map = self.type_map();
        for x in self.skillsets.iter_mut() {
            x.resolve_type(&map)?;
        }
        Ok(())
    }

    fn resolve_resource(&mut self) -> Result<(), RlError> {
        for x in self.skillsets.iter_mut() {
            x.resolve_resource()?;
        }
        Ok(())
    }

    fn resolve_state(&mut self) -> Result<(), RlError> {
        for x in self.skillsets.iter_mut() {
            x.resolve_state()?;
        }
        Ok(())
    }

    //----------  ----------

    pub fn to_lang(&self) -> String {
        let mut s = String::new();
        // ----- Type -----
        match self.types.len() {
            0 => {}
            1 => s.push_str(&format!("type {}\n", self.types().get(0).unwrap())),
            _ => {
                s.push_str(&format!("type {{\n"));
                for t in self.types() {
                    s.push_str(&format!("\t{}\n", t));
                }
                s.push_str("}\n");
            }
        }
        // ----- Skillset -----
        for skillset in self.skillsets.iter() {
            s.push_str(&skillset.to_lang(self));
        }
        //
        s
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            types: Vec::new(),
            skillsets: Vec::new(),
        }
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<TypeId, RlType> for Model {
    fn get(&self, id: TypeId) -> Option<&RlType> {
        self.get_type(id)
    }
}
impl GetFromId<SkillsetId, Skillset> for Model {
    fn get(&self, id: SkillsetId) -> Option<&Skillset> {
        self.get_skillset(id)
    }
}

// From Skillset
impl GetFromId<DataId, Data> for Model {
    fn get(&self, id: DataId) -> Option<&Data> {
        let DataId(skillset_id, _) = id;
        let skillset = self.get(skillset_id)?;
        skillset.get(id)
    }
}
impl GetFromId<ResourceId, Resource> for Model {
    fn get(&self, id: ResourceId) -> Option<&Resource> {
        let ResourceId(skillset_id, _) = id;
        let skillset = self.get(skillset_id)?;
        skillset.get(id)
    }
}
impl GetFromId<StateId, State> for Model {
    fn get(&self, id: StateId) -> Option<&State> {
        let StateId(resource_id, _) = id;
        let resource = self.get(resource_id)?;
        resource.get(id)
    }
}
impl GetFromId<EventId, Event> for Model {
    fn get(&self, id: EventId) -> Option<&Event> {
        let EventId(skillset_id, _) = id;
        let skillset = self.get(skillset_id)?;
        skillset.get(id)
    }
}
impl GetFromId<SkillId, Skill> for Model {
    fn get(&self, id: SkillId) -> Option<&Skill> {
        let SkillId(skillset_id, _) = id;
        let skillset = self.get(skillset_id)?;
        skillset.get(id)
    }
}
impl GetFromId<PreconditionId, Precondition> for Model {
    fn get(&self, id: PreconditionId) -> Option<&Precondition> {
        let PreconditionId(skill_id, _) = id;
        let skill = self.get(skill_id)?;
        skill.get(id)
    }
}
impl GetFromId<InvariantId, Invariant> for Model {
    fn get(&self, id: InvariantId) -> Option<&Invariant> {
        let InvariantId(skill_id, _) = id;
        let skill = self.get(skill_id)?;
        skill.get(id)
    }
}
impl GetFromId<SuccessId, Success> for Model {
    fn get(&self, id: SuccessId) -> Option<&Success> {
        let SuccessId(skill_id, _) = id;
        let skill = self.get(skill_id)?;
        skill.get(id)
    }
}
impl GetFromId<FailureId, Failure> for Model {
    fn get(&self, id: FailureId) -> Option<&Failure> {
        let FailureId(skill_id, _) = id;
        let skill = self.get(skill_id)?;
        skill.get(id)
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_lang())
    }
}
