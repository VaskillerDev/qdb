extern crate yaml_rust;

use self::yaml_rust::Yaml;
use crate::containers::container::Container;
use clap::{App, ArgMatches, YamlLoader};
use std::fs::File;
use std::io::Error;
use std::io::Read;

#[doc = "Load yaml-file with App metadata for clap's\n
raw_yaml - string arg for accumulate value from function\n"]
pub fn load_args(mut raw_yaml: String) -> String {
    let mut container = Container::new("resource");
    container.load_dir();
    let path_app = container.get("app-conf.yml").unwrap();

    std::fs::File::open(path_app)
        .unwrap()
        .read_to_string(&mut raw_yaml);

    raw_yaml
}

#[doc = "Load yaml-file with list of operator's\n
path - path to file"]
pub fn load_operators(path: &String) -> Result<Vec<Yaml>, Error> {
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

#[cfg(test)]
mod test {
    use crate::args_loader::{load_args, load_operators};
    use clap::{App, YamlLoader};

    #[test]
    fn test_app_args_loader() -> std::io::Result<()> {
        let mut app_string = String::new();
        app_string = load_args(app_string);
        let yaml = YamlLoader::load_from_str(app_string.as_str());
        match &yaml {
            Ok(_) => println!("Vec<Yaml> loaded"),
            Err(e) => println!("{:?}",e)
        }
        let yaml = &yaml.unwrap()[0];
        let matches = &App::from_yaml(yaml).get_matches();
        Ok(())
    }
}
