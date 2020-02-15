use yaml_rust::Yaml;

#[derive(Debug)]
pub struct AppInfo {
    name: String,
    version: String,
    debug: bool,
    operators: Vec<Yaml>,
    exec: bool,
    pub query: String,
}

impl AppInfo {
    pub fn new(
        name: String,
        version: String,
        debug: bool,
        operators: Vec<Yaml>,
        exec: bool,
    ) -> AppInfo {
        AppInfo {
            name,
            version,
            debug,
            operators,
            exec,
            query: "".to_string(),
        }
    }

    pub fn run(&self) {}

    fn inject() {}
}
