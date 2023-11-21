#[macro_use]
extern crate lalrpop_util;

#[macro_use]
extern crate log;

pub mod model;
pub mod parser;

pub fn process_file(model: &mut model::Model, filename: &str) -> Result<(), parser::RlError> {
    let mut parser = parser::Parser::new(model);
    // Parsing
    match parser::parse_file(model, filename) {
        Ok(_) => info!("Parsing OK"),
        Err(e) => {
            error!("{}", e);
            return Err(e);
        }
    }
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
    Ok(())
}

pub fn load_model(file: &str) -> Result<model::Model, parser::RlError> {
    let mut model = model::Model::default();
    match process_file(&mut model, file) {
        Ok(_) => Ok(model),
        Err(e) => Err(e),
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
