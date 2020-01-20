extern crate yaml_rust;

use self::yaml_rust::Yaml;
use crate::containers::container::Container;
use clap::{App, YamlLoader};
use std::fs::File;
use std::io::Error;
use std::io::Read;

#[doc = "Load yaml-file with App metadata for clap's\n
raw_yaml - string arg for accumulate value from function\n"]
fn load_args(mut raw_yaml: String) -> String {
    const CONTAINER_NAME: &str = "resource";
    const CONTAINER_KEY: &str = "app-conf.yml";
    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();
    let path_app = container.get(CONTAINER_KEY);
    assert_eq!(
        path_app.is_some(),
        true,
        "Key '{}' in container '{}' not found",
        CONTAINER_KEY,
        CONTAINER_NAME
    );
    let path_app = path_app.unwrap();
    let file = std::fs::File::open(path_app);
    assert_eq!(
        file.is_ok(),
        true,
        "File {} in path {} not found",
        CONTAINER_KEY,
        path_app
    );
    match file.unwrap().read_to_string(&mut raw_yaml) {
        Ok(_) => {}
        Err(e) => println!("{}", e),
    }
    raw_yaml
}

#[doc = "Load yaml-file with list of operator's\n
path - path to file"]
fn load_operators(path: &String) -> Result<Vec<Yaml>, Error> {
    assert!(
        std::path::Path::new(path).exists(),
        "File should not exists"
    );
    let mut file = File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    let docs: Vec<yaml_rust::Yaml> = match yaml_rust::YamlLoader::load_from_str(content.as_str()) {
        Ok(docs) => docs,
        Err(_) => panic!("Yaml-file is broken"),
    };
    Ok(docs)
}

#[doc = "load_args && load_operators"]
pub fn exec() {
    let mut app_string = String::new();
    app_string = load_args(app_string);
    let yaml = &YamlLoader::load_from_str(app_string.as_str()).unwrap()[0];
    let matches = &App::from_yaml(&yaml).get_matches();

    let app_config = matches.value_of("config").unwrap_or("");
    let app_name = &yaml["name"].as_str().unwrap_or("");
    println!("{}: load...", &app_name);

    let _operators = load_operators(&app_config.to_string());

    println!("{}: has been load", &app_name);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_args() -> std::io::Result<()> {
        let mut app_string = String::new();
        app_string = load_args(app_string);
        let yaml = YamlLoader::load_from_str(app_string.as_str());
        assert_eq!(yaml.is_ok(), true, "It is not yaml file");
        let yaml = &yaml.unwrap();
        assert_ne!(yaml.is_empty(), true, "It is empty file");
        Ok(())
    }
}
