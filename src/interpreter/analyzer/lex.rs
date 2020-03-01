extern crate regex;

use self::regex::Error;
use crate::containers::container::Container;
use regex::Regex;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs::File;
use yaml_rust::{Yaml, YamlLoader};

const CONTAINER_NAME: &str = "resource";
const CONTAINER_KEY: &str = "qdb-conf.yml";

#[derive(PartialEq, Debug)]
enum TextProcRule {
    FindFuncs,
    FindArgsInFuncs,
    Done,
}

impl TextProcRule {
    pub fn next(&mut self) {
        match self {
            TextProcRule::FindFuncs => *self = TextProcRule::FindArgsInFuncs,
            TextProcRule::FindArgsInFuncs => *self = TextProcRule::Done,
            TextProcRule::Done => {}
        }
    }
}

// load qdb_conf.yml
fn load_qdb_conf() -> Box<Yaml> {
    use std::io::Read;

    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();

    let path = container.get(CONTAINER_KEY).unwrap();
    let mut file = File::open(path).unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data);

    let docs = YamlLoader::load_from_str(&data).unwrap();
    let docs = docs[0].borrow();

    Box::new(docs.clone())
}

// return values from qdb_conf.yml
fn build_lexems<T: IntoIterator<Item = String> + Debug>(
    yaml: &Box<Yaml>,
    section: &str,
    query_values: T,
) -> Vec<String> {
    let section = yaml[section].as_hash().unwrap();
    let section: HashMap<&Yaml, &Yaml> = section.iter().clone().collect();
    let mut result: Vec<String> = Vec::new();

    for i in query_values {
        result.push(String::from(
            section
                .get_key_value(&Yaml::from_str(i.as_str()))
                .unwrap()
                .1
                .as_str()
                .unwrap(),
        ));
    }
    result
}

// build_lexems -> load_qdb_conf
fn execute_text_processing(
    yaml: &Box<Yaml>,
    text_proc_rule: &TextProcRule,
) -> Option<Result<Regex, Error>> {
    let mut accumulator: Vec<String> = Vec::new();
    match text_proc_rule {
        TextProcRule::FindFuncs => {
            accumulator.push("br_open".to_string());
            accumulator.push("br_close".to_string());
            accumulator = build_lexems(yaml, "operator", accumulator);
            let raw_regex = format!(
                r"\s*[A-Za-z_][A-Za-z0-9_]*\{}[^0-9()]*\{};",
                accumulator[0], accumulator[1]
            );
            Some(Regex::new(raw_regex.as_str()))
        }
        TextProcRule::FindArgsInFuncs => Some(Regex::new(
            r"([A-Za-z_][A-Za-z_0-9]*\s*):\s*([A-Za-z_][A-Za-z_0-9]*)",
        )),
        TextProcRule::Done => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_from_file() {
        let qdb_conf = load_qdb_conf();
        let mut text_proc_rule = TextProcRule::FindFuncs;

        while text_proc_rule != TextProcRule::Done {
            let res = execute_text_processing(&qdb_conf, &text_proc_rule);
            text_proc_rule.next();
            // next(& mut text_proc_rule);
            println!("{:?}", res.unwrap());
        }
    }
}
