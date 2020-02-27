use crate::containers::container::Container;
use std::borrow::Borrow;
use std::fs::File;
use yaml_rust::{Yaml, YamlLoader};
use std::collections::{HashMap, HashSet};

impl Token {}

pub struct Token {
    value: String,
    transform: dyn FnOnce(String) -> String,
}

// load qdb_conf.yml
fn load_qdb_conf() -> Box<Yaml> {
    use std::io::Read;
    const CONTAINER_NAME: &str = "resource";

    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();

    let path = container.get("qdb-conf.yml").unwrap();
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data);

    let docs = YamlLoader::load_from_str(&data).unwrap();
    let docs = docs[0].borrow();

    Box::new(docs.clone())
}

fn inspect_keywords(yaml : Box<Yaml>) {
   let operator = yaml["operator"].as_hash().unwrap();

   for i in operator {
       println!("{:?}",i.0);
   }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_from_file() {
        let qdb_conf = load_qdb_conf();
        inspect_keywords(qdb_conf);

    }
}
