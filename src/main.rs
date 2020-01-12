extern crate clap;

mod args_loader;
mod containers;
mod operators;

use crate::args_loader::{load_args, load_operators};
use clap::{App, YamlLoader};

/// qdb -c / --config (path)
fn main() -> std::io::Result<()> {
    let mut app_string = String::new();
    app_string = load_args(app_string);
    let yaml = &YamlLoader::load_from_str(app_string.as_str()).unwrap()[0];
    let matches = &App::from_yaml(&yaml).get_matches();

    let app_config = matches.value_of("config").unwrap_or("");
    let app_name = &yaml["name"].as_str().unwrap_or("");
    println!("{}: load...", &app_name);

    let operators = load_operators(&app_config.to_string())?;

    println!("{}: has been load", &app_name);
    Ok(())
}
