extern crate  yaml_rust;

use yaml_rust::{YamlLoader};
use std::io::Read;
use std::fs::File;
use self::yaml_rust::Yaml;
use std::io::Error;

#[doc = "Load yaml-file with list of operator's\n
path - path to file"]
pub fn load_operators ( path: &String) -> Result < Vec<Yaml>,Error > {

    assert!(std::path::Path::new(path).exists(),"File should not exists");
    let mut file = File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let docs = match YamlLoader::load_from_str(content.as_str()) {
        Ok(docs) => docs,
        Err(_)=>  panic!("Yaml-file is broken"),
    };
    Ok(docs)

}

#[doc = "Load arguments"]
pub fn load_arguments () -> Vec<String>  {
    let mut arguments = Vec::new();
    for argument in std::env::args() {
        arguments.push(argument.to_string());
    }
    if arguments.len() < 2 { panic!("Argument 'path' not found");}
    arguments
}