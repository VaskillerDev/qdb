#[macro_use]
extern crate clap;

mod operators;
mod containers;
mod args_loader;
use clap::App;
use std::path::{Path};
use crate::args_loader::load_operators;

const RESOURCE_DIR : &str = "resource";

/// qdb -c / --config (path)
fn main() -> std::io::Result<()>{

    let yaml = load_yaml!("../resource/app.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let config = matches.value_of("config").unwrap_or("");
    let app_name = matches.value_of("name").unwrap_or("");

    println!("{}: load...",app_name);

    let operators =  load_operators(&config.to_string())?;

    println!("{}: has been load",app_name);
    Ok(())
}