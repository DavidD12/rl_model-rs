use clap::Parser;
use rl_model::*;
use std::env;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// robot language file
    #[arg(short, long)]
    file: String,
    /// verbose level
    #[arg(short, long, default_value_t = 1)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    if args.verbose > 0 {
        //
        if env::var("RUST_LOG").is_err() {
            env::set_var("RUST_LOG", "info")
        }
        // pretty_env_logger::init();
        env_logger::init();
    }
    //
    if let Ok(model) = load_skillset(&args.file) {
        if args.verbose >= 3 {
            // println!("{:#?}", model);
            println!("{}", model);
        }
        // if check(&model) {
        //     // TODO
        // }
    }
}
