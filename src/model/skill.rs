use super::*;
use crate::parser::{Position, RlError};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct SkillId(pub usize);
impl Id for SkillId {
    fn index(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct Skill {
    id: SkillId,
    name: String,
    inputs: Vec<Variable>,
    outputs: Vec<Variable>,
    preconditions: Vec<Precondition>,
    start: Vec<Effect>,
    invariants: Vec<Invariant>,
    progress: Option<Progress>,
    interrupt: Option<Interrupt>,
    successes: Vec<Success>,
    failures: Vec<Failure>,
    position: Option<Position>,
}

impl Skill {
    pub fn new<S: Into<String>>(name: S, position: Option<Position>) -> Self {
        let id = SkillId::default();
        let name = name.into();
        Self {
            id,
            name,
            inputs: Vec::new(),
            outputs: Vec::new(),
            preconditions: Vec::new(),
            start: Vec::new(),
            invariants: Vec::new(),
            progress: None,
            interrupt: None,
            successes: Vec::new(),
            failures: Vec::new(),
            position,
        }
    }

    //---------- Input ----------

    pub fn inputs(&self) -> &Vec<Variable> {
        &self.inputs
    }

    pub fn add_input(&mut self, input: Variable) {
        self.inputs.push(input);
    }

    //---------- Output ----------

    pub fn outputs(&self) -> &Vec<Variable> {
        &self.outputs
    }

    pub fn add_output(&mut self, output: Variable) {
        self.outputs.push(output);
    }

    //---------- Precondition ----------

    pub fn preconditions(&self) -> &Vec<Precondition> {
        &self.preconditions
    }

    pub fn add_precondition(&mut self, mut precondition: Precondition) -> PreconditionId {
        let id = PreconditionId(self.id, self.preconditions.len());
        precondition.set_id(id);
        self.preconditions.push(precondition);
        id
    }

    pub fn get_precondition(&self, id: PreconditionId) -> Option<&Precondition> {
        let PreconditionId(skill_id, n) = id;
        if self.id != skill_id {
            None
        } else {
            self.preconditions.get(n)
        }
    }

    //---------- Start ----------

    pub fn start(&self) -> &Vec<Effect> {
        &self.start
    }

    pub fn set_start(&mut self, effects: Vec<Effect>) {
        self.start = effects;
    }

    //---------- Invariant ----------

    pub fn invariants(&self) -> &Vec<Invariant> {
        &self.invariants
    }

    pub fn add_invariant(&mut self, mut invariant: Invariant) -> InvariantId {
        let id = InvariantId(self.id, self.invariants.len());
        invariant.set_id(id);
        self.invariants.push(invariant);
        id
    }

    pub fn get_invariant(&self, id: InvariantId) -> Option<&Invariant> {
        let InvariantId(skill_id, n) = id;
        if self.id != skill_id {
            None
        } else {
            self.invariants.get(n)
        }
    }

    //---------- Progress ----------

    pub fn progress(&self) -> &Option<Progress> {
        &self.progress
    }

    pub fn set_progress(&mut self, progress: Progress) {
        self.progress = Some(progress)
    }

    //---------- Interrupt ----------

    pub fn interrupt(&self) -> &Option<Interrupt> {
        &self.interrupt
    }

    pub fn set_interrupt(&mut self, interrupt: Interrupt) {
        self.interrupt = Some(interrupt);
    }

    //---------- Success ----------

    pub fn successes(&self) -> &Vec<Success> {
        &self.successes
    }

    pub fn add_success(&mut self, mut success: Success) -> SuccessId {
        let id = SuccessId(self.id, self.successes.len());
        success.set_id(id);
        self.successes.push(success);
        id
    }

    pub fn get_success(&self, id: SuccessId) -> Option<&Success> {
        let SuccessId(skill_id, n) = id;
        if self.id != skill_id {
            None
        } else {
            self.successes.get(n)
        }
    }

    //---------- Failure ----------

    pub fn failures(&self) -> &Vec<Failure> {
        &self.failures
    }

    pub fn add_failure(&mut self, mut failure: Failure) -> FailureId {
        let id = FailureId(self.id, self.failures.len());
        failure.set_id(id);
        self.failures.push(failure);
        id
    }

    pub fn get_failure(&self, id: FailureId) -> Option<&Failure> {
        let FailureId(skill_id, n) = id;
        if self.id != skill_id {
            None
        } else {
            self.failures.get(n)
        }
    }

    //---------- Duplicate ----------

    pub fn input_naming(&self) -> Vec<Naming> {
        self.inputs
            .iter()
            .map(|x| (x.name().into(), x.position()))
            .collect()
    }
    pub fn output_naming(&self) -> Vec<Naming> {
        self.outputs
            .iter()
            .map(|x| (x.name().into(), x.position()))
            .collect()
    }
    pub fn precondition_naming(&self) -> Vec<Naming> {
        self.preconditions.iter().map(|x| x.naming()).collect()
    }
    pub fn invariant_naming(&self) -> Vec<Naming> {
        self.invariants.iter().map(|x| x.naming()).collect()
    }
    pub fn success_naming(&self) -> Vec<Naming> {
        self.successes.iter().map(|x| x.naming()).collect()
    }
    pub fn failure_naming(&self) -> Vec<Naming> {
        self.failures.iter().map(|x| x.naming()).collect()
    }

    pub fn duplicate(&self, skillset: &Skillset) -> Result<(), RlError> {
        let types = skillset.type_naming();

        // Input
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.input_naming().into_iter())
                .collect(),
        )?;
        // Output
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.output_naming().into_iter())
                .collect(),
        )?;
        // Precondition
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.precondition_naming().into_iter())
                .collect(),
        )?;
        // Invariant
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.invariant_naming().into_iter())
                .collect(),
        )?;
        // Success
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.success_naming().into_iter())
                .collect(),
        )?;
        // Failure
        check_duplicate(
            types
                .clone()
                .into_iter()
                .chain(self.failure_naming().into_iter())
                .collect(),
        )?;

        Ok(())
    }

    //---------- Resolve ----------

    pub fn resolve_type(&mut self, map: &HashMap<String, TypeId>) -> Result<(), RlError> {
        // Input
        for x in self.inputs.iter_mut() {
            x.resolve_type(map)?;
        }
        // Output
        for x in self.outputs.iter_mut() {
            x.resolve_type(map)?;
        }
        // Progress
        if let Some(progress) = &mut self.progress {
            progress.resolve_type(map)?;
        }
        Ok(())
    }

    pub fn resolve_resource(&mut self, map: &HashMap<String, ResourceId>) -> Result<(), RlError> {
        // Precondition
        for x in self.preconditions.iter_mut() {
            x.resolve_resource(map)?;
        }
        // Start
        for x in self.start.iter_mut() {
            x.resolve_resource(map)?;
        }
        // Invariant
        for x in self.invariants.iter_mut() {
            x.resolve_resource(map)?;
        }
        // Interrupt
        if let Some(i) = &mut self.interrupt {
            i.resolve_resource(map)?;
        }
        // Success
        for x in self.successes.iter_mut() {
            x.resolve_resource(map)?;
        }
        // Failure
        for x in self.failures.iter_mut() {
            x.resolve_resource(map)?;
        }
        Ok(())
    }

    pub fn resolve_state(&mut self, map: &HashMap<String, StateId>) -> Result<(), RlError> {
        // Precondition
        for x in self.preconditions.iter_mut() {
            x.resolve_state(map)?;
        }
        // Start
        for x in self.start.iter_mut() {
            x.resolve_state(map)?;
        }
        // Invariant
        for x in self.invariants.iter_mut() {
            x.resolve_state(map)?;
        }
        // Interrupt
        if let Some(i) = &mut self.interrupt {
            i.resolve_state(map)?;
        }
        // Success
        for x in self.successes.iter_mut() {
            x.resolve_state(map)?;
        }
        // Failure
        for x in self.failures.iter_mut() {
            x.resolve_state(map)?;
        }
        Ok(())
    }
}

impl Named<SkillId> for Skill {
    fn id(&self) -> SkillId {
        self.id
    }
    fn set_id(&mut self, id: SkillId) {
        self.id = id;
        for x in self.preconditions.iter_mut() {
            let PreconditionId(_, index) = x.id();
            x.set_id(PreconditionId(id, index));
        }
        for x in self.invariants.iter_mut() {
            let InvariantId(_, index) = x.id();
            x.set_id(InvariantId(id, index));
        }
        for x in self.successes.iter_mut() {
            let SuccessId(_, index) = x.id();
            x.set_id(SuccessId(id, index));
        }
        for x in self.failures.iter_mut() {
            let FailureId(_, index) = x.id();
            x.set_id(FailureId(id, index));
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    fn position(&self) -> Option<Position> {
        self.position.clone()
    }
}

impl GetFromId<PreconditionId, Precondition> for Skill {
    fn get(&self, id: PreconditionId) -> Option<&Precondition> {
        self.get_precondition(id)
    }
}
impl GetFromId<InvariantId, Invariant> for Skill {
    fn get(&self, id: InvariantId) -> Option<&Invariant> {
        self.get_invariant(id)
    }
}
impl GetFromId<SuccessId, Success> for Skill {
    fn get(&self, id: SuccessId) -> Option<&Success> {
        self.get_success(id)
    }
}
impl GetFromId<FailureId, Failure> for Skill {
    fn get(&self, id: FailureId) -> Option<&Failure> {
        self.get_failure(id)
    }
}

impl ToLang for Skill {
    fn to_lang(&self, skillset: &Skillset) -> String {
        let mut s = String::new();
        s.push_str(&format!("\t\t{} {{\n", self.name));
        // Input
        if !self.inputs.is_empty() {
            s.push_str("\t\t\tinput {\n");
            for x in self.inputs.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Output
        if !self.inputs.is_empty() {
            s.push_str("\t\t\toutput {\n");
            for x in self.outputs.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Precondition
        if !self.preconditions.is_empty() {
            s.push_str("\t\t\tprecondition {\n");
            for x in self.preconditions.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Start
        if !self.start.is_empty() {
            s.push_str("\t\t\tstart {\n");
            for x in self.start.iter() {
                s.push_str(&format!("\t\t\t\t{}\n", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Invariant
        if !self.invariants.is_empty() {
            s.push_str("\t\t\tinvariant {\n");
            for x in self.invariants.iter() {
                s.push_str(&format!("\t\t\t\t{}", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Progress
        if let Some(progress) = &self.progress {
            s.push_str(&progress.to_lang(skillset));
        }
        // Interrupt
        if let Some(i) = &self.interrupt {
            s.push_str(&i.to_lang(skillset));
        }
        // Success
        if !self.successes.is_empty() {
            s.push_str("\t\t\tsuccess {\n");
            for x in self.successes.iter() {
                s.push_str(&format!("\t\t\t\t{}", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t}\n");
        }
        // Failure
        if !self.failures.is_empty() {
            s.push_str("\t\t\tfailure {\n");
            for x in self.failures.iter() {
                s.push_str(&format!("\t\t\t\t{}", x.to_lang(skillset)))
            }
            s.push_str("\t\t\t}\n");
        }
        //
        s.push_str("\t\t}\n");
        s
    }
}

impl std::fmt::Display for Skill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}
