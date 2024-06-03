pub mod reference;
pub use reference::*;

// pub mod model;
// pub use model::*;

pub mod rl_type;
pub use rl_type::*;

pub mod skillset;
pub use skillset::*;

pub mod data;
pub use data::*;

pub mod resource;
pub use resource::*;
pub mod state;
pub use state::*;
pub mod transition;
pub use transition::*;

pub mod event;
pub use event::*;

pub mod skill;
pub use skill::*;
// pub mod input;
// pub use input::*;
// pub mod output;
// pub use output::*;

pub mod precondition;
pub use precondition::*;

pub mod postcondition;
pub use postcondition::*;

pub mod invariant;
pub use invariant::*;

pub mod progress;
pub use progress::*;

pub mod interrupt;
pub use interrupt::*;

pub mod terminate;
pub use terminate::*;

pub mod effect;
pub use effect::*;

pub mod variable;
pub use variable::*;

pub mod expr;
pub use expr::*;

use crate::parser::{Position, RlError};

pub trait ToLang {
    fn to_lang(&self, skillset: &Skillset) -> String;
}

//------------------------- Id -------------------------

pub trait Id: Clone + Copy + PartialEq + Eq + core::hash::Hash + std::fmt::Debug + Default {
    fn index(&self) -> usize;
}

pub trait GetFromId<I: Id, T> {
    fn get(&self, i: I) -> Option<&T>;
}

//------------------------- Named -------------------------

pub trait Named<I: Id> {
    fn id(&self) -> I;
    fn set_id(&mut self, id: I);
    fn name(&self) -> &str;
    fn position(&self) -> Option<Position>;
    fn naming(&self) -> Naming {
        (self.name().into(), self.position())
    }
}

pub type Naming = (String, Option<Position>);

pub fn check_duplicate(names: Vec<Naming>) -> Result<(), RlError> {
    for (i, (n1, p1)) in names.iter().enumerate() {
        for (n2, p2) in names.iter().skip(i + 1) {
            if n1 == n2 {
                return Err(RlError::Duplicate {
                    name: n1.clone(),
                    first: p1.clone(),
                    second: p2.clone(),
                });
            }
        }
    }
    Ok(())
}
