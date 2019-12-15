#[macro_use]
extern crate clap;

mod args_loader;
mod operators;

use std::env;
use crate::args_loader::{load_operators};
use clap::App;

/// qdb (path)
fn main() -> std::io::Result<()>{

    let yaml = load_yaml!("app.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let r = matches.value_of("config").unwrap_or("");
    println!("[qdb-core]: load...");
    let operators = load_operators(&r.to_string())?;
    for op in operators {
        println!("{:?}",op);
    }


    println!("[qdb-core]: has been load");

    Ok(())
}