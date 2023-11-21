#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub mod model;
pub mod parser;
use crate::model::Model;

pub fn load_model(filename: &str) -> Result<Model, parser::RlError> {
    // Parsing
    match parser::parse_file(filename) {
        Ok(mut model) => {
            info!("Parsing OK");
            // Duplicate
            match model.duplicate() {
                Ok(_) => info!("Duplicate OK"),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            }
            // Resolve
            match model.resolve() {
                Ok(_) => info!("Resolve OK"),
                Err(e) => {
                    error!("{}", e);
                    return Err(e);
                }
            }
            //
            Ok(model)
        }
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }
}

// pub fn check(model: &model::Model) -> bool {
//     let errors = check_model(model);
//     if errors.is_empty() {
//         info!("Verification OK");
//         true
//     } else {
//         for e in errors.iter() {
//             error!("{}", e.to_lang(model));
//         }
//         false
//     }
// }
