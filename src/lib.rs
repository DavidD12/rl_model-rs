#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub mod model;
pub mod parser;
use crate::model::Skillset;

pub fn load_skillset(filename: &str) -> Result<Skillset, parser::RlError> {
    // Parsing
    match parser::parse_file(filename) {
        Ok(mut skillset) => {
            info!("Parsing OK");
            // Duplicate
            match skillset.duplicate() {
                Ok(_) => info!("Duplicate OK"),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            }
            // Resolve
            match skillset.resolve() {
                Ok(_) => info!("Resolve OK"),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            }
            //
            Ok(skillset)
        }
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }
}

// pub fn check(skillset: &skillset::Model) -> bool {
//     let errors = check_skillset(skillset);
//     if errors.is_empty() {
//         info!("Verification OK");
//         true
//     } else {
//         for e in errors.iter() {
//             error!("{}", e.to_lang(skillset));
//         }
//         false
//     }
// }
