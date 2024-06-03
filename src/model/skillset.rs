use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Skillset {
    name: String,
    types: Vec<RlType>,
    data: Vec<Data>,
    resources: Vec<Resource>,
    events: Vec<Event>,
    skills: Vec<Skill>,
    position: Option<Position>,
}

impl Skillset {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let name = name.into();
        Self {
            name,
            types: Default::default(),
            data: Default::default(),
            resources: Default::default(),
            events: Default::default(),
            skills: Default::default(),
            position,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> Option<Position> {
        self.position.clone()
    }

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

    //---------- Data ----------

    pub fn data(&self) -> &Vec<Data> {
        &self.data
    }

    pub fn get_data(&self, id: DataId) -> Option<&Data> {
        self.data.get(id.index())
    }

    pub fn add_data(&mut self, mut data: Data) -> DataId {
        let id = DataId(self.data.len());
        data.set_id(id);
        self.data.push(data);
        id
    }

    //---------- Resource ----------

    pub fn resources(&self) -> &Vec<Resource> {
        &self.resources
    }

    pub fn get_resource(&self, id: ResourceId) -> Option<&Resource> {
        self.resources.get(id.index())
    }

    pub fn add_resource(&mut self, mut resource: Resource) -> ResourceId {
        let id = ResourceId(self.resources.len());
        resource.set_id(id);
        self.resources.push(resource);
        id
    }

    pub fn resource_map(&self) -> HashMap<String, ResourceId> {
        let mut map = HashMap::new();
        for x in self.resources.iter() {
            map.insert(x.name().into(), x.id());
        }
        map
    }

    //---------- State ----------

    pub fn get_state(&self, id: StateId) -> Option<&State> {
        let resource = self.get_resource(id.resource())?;
        resource.get_state(id)
    }

    pub fn state_map(&self) -> HashMap<String, StateId> {
        let mut map = HashMap::new();
        for x in self.resources.iter() {
            for y in x.states().iter() {
                map.insert(y.name().into(), y.id());
            }
        }
        map
    }

    //---------- Event ----------

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn get_event(&self, id: EventId) -> Option<&Event> {
        self.events.get(id.index())
    }

    pub fn add_event(&mut self, mut event: Event) -> EventId {
        let id = EventId(self.events.len());
        event.set_id(id);
        self.events.push(event);
        id
    }

    //---------- Skill ----------

    pub fn skills(&self) -> &Vec<Skill> {
        &self.skills
    }

    pub fn get_skill(&self, id: SkillId) -> Option<&Skill> {
        self.skills.get(id.index())
    }

    pub fn add_skill(&mut self, mut skill: Skill) -> SkillId {
        let id = SkillId(self.skills.len());
        skill.set_id(id);
        self.skills.push(skill);
        id
    }

    //---------- Duplicate ----------

    pub fn type_naming(&self) -> Vec<Naming> {
        self.types.iter().map(|x| x.naming()).collect()
    }

    pub fn data_naming(&self) -> Vec<Naming> {
        self.data.iter().map(|x| x.naming()).collect()
    }
    pub fn resource_naming(&self) -> Vec<Naming> {
        let mut v = Vec::new();
        for x in self.resources.iter() {
            v.push((x.name().into(), x.position()));
            v.extend(x.names());
        }
        v
    }
    pub fn event_naming(&self) -> Vec<Naming> {
        self.events.iter().map(|x| x.naming()).collect()
    }
    pub fn skill_naming(&self) -> Vec<Naming> {
        self.skills.iter().map(|x| x.naming()).collect()
    }

    pub fn duplicate(&self) -> Result<(), RlError> {
        let types = self.type_naming();

        // Data
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.data_naming().into_iter())
                .collect(),
        )?;
        // Resource
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.resource_naming().into_iter())
                .collect(),
        )?;
        // Event
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.event_naming().into_iter())
                .collect(),
        )?;
        // Skill
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.skill_naming().into_iter())
                .collect(),
        )?;

        // Skill
        for x in self.skills.iter() {
            x.duplicate(self)?;
        }

        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve(&mut self) -> Result<(), RlError> {
        self.resolve_type()?;
        self.resolve_resource()?;
        self.resolve_state()
    }

    pub fn resolve_type(&mut self) -> Result<(), RlError> {
        let map = self.type_map();
        // Data
        for x in self.data.iter_mut() {
            x.resolve_type(&map)?;
        }
        // Skill
        for x in self.skills.iter_mut() {
            x.resolve_type(&map)?;
        }
        Ok(())
    }

    pub fn resolve_resource(&mut self) -> Result<(), RlError> {
        let map = self.resource_map();
        // Event
        for x in self.events.iter_mut() {
            x.resolve_resource(&map)?;
        }
        // Skill
        for x in self.skills.iter_mut() {
            x.resolve_resource(&map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self) -> Result<(), RlError> {
        // Resource
        for x in self.resources.iter_mut() {
            x.resolve_state()?;
        }
        // Event
        let map = self.state_map();
        for x in self.events.iter_mut() {
            x.resolve_state(&map)?;
        }
        // Skill
        let map = self.state_map();
        for x in self.skills.iter_mut() {
            x.resolve_state(&map)?;
        }
        Ok(())
    }
}

//------------------------- Get From Id -------------------------

impl GetFromId<TypeId, RlType> for Skillset {
    fn get(&self, id: TypeId) -> Option<&RlType> {
        self.get_type(id)
    }
}

impl GetFromId<DataId, Data> for Skillset {
    fn get(&self, id: DataId) -> Option<&Data> {
        self.get_data(id)
    }
}
impl GetFromId<ResourceId, Resource> for Skillset {
    fn get(&self, id: ResourceId) -> Option<&Resource> {
        self.get_resource(id)
    }
}
impl GetFromId<StateId, State> for Skillset {
    fn get(&self, id: StateId) -> Option<&State> {
        self.get_state(id)
    }
}
impl GetFromId<EventId, Event> for Skillset {
    fn get(&self, id: EventId) -> Option<&Event> {
        self.get_event(id)
    }
}
impl GetFromId<SkillId, Skill> for Skillset {
    fn get(&self, id: SkillId) -> Option<&Skill> {
        self.get_skill(id)
    }
}
impl GetFromId<PreconditionId, Precondition> for Skillset {
    fn get(&self, id: PreconditionId) -> Option<&Precondition> {
        let skill = self.get(id.skill())?;
        skill.get(id)
    }
}
impl GetFromId<InvariantId, Invariant> for Skillset {
    fn get(&self, id: InvariantId) -> Option<&Invariant> {
        let skill = self.get(id.skill())?;
        skill.get(id)
    }
}
impl GetFromId<SuccessId, Success> for Skillset {
    fn get(&self, id: SuccessId) -> Option<&Success> {
        let skill = self.get(id.skill())?;
        skill.get(id)
    }
}
impl GetFromId<FailureId, Failure> for Skillset {
    fn get(&self, id: FailureId) -> Option<&Failure> {
        let skill = self.get(id.skill())?;
        skill.get(id)
    }
}

//------------------------- Display -------------------------

impl std::fmt::Display for Skillset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "skillset {}", self.name)?;
        // Types
        if let Some((first, others)) = self.types.split_first() {
            write!(f, "<{}", first.name())?;
            for t in others {
                write!(f, ", {}", t.name())?;
            }
            write!(f, ">")?;
        }
        write!(f, " {{\n")?;
        // Data
        if !self.data.is_empty() {
            write!(f, "\tdata {{\n")?;
            for x in self.data.iter() {
                write!(f, "\t\t{}", &x.to_lang(self))?;
            }
            write!(f, "\t}}\n")?;
        }
        // Resource
        if !self.resources.is_empty() {
            write!(f, "\tresource {{\n")?;
            for x in self.resources.iter() {
                write!(f, "{}", x.to_lang(self))?;
            }
            write!(f, "\t}}\n")?;
        }
        // Event
        if !self.events.is_empty() {
            write!(f, "\tevent {{\n")?;
            for x in self.events.iter() {
                write!(f, "{}", x.to_lang(self))?;
            }
            write!(f, "\t}}\n")?;
        }
        // Skill
        if !self.skills.is_empty() {
            write!(f, "\tskill {{\n")?;
            for x in self.skills.iter() {
                write!(f, "{}", x.to_lang(self))?;
            }
            write!(f, "\t}}\n")?;
        }
        //
        write!(f, "}}\n")?;
        //
        Ok(())
    }
}
