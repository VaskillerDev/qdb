use std::collections::HashMap;
use std::env::current_dir;
use std::ffi::OsString;

// Container is instance for get/set values for safety access data
#[doc = "Based Container - key/value storage. Represent directories patches from named directory"]
pub struct Container {
    name: String,
    value: HashMap<String, String>,
    root: Option<OsString>,
}

// Implementation for Based Container instance
impl Container {
    #[doc = "Create new Container with name.\n
    name - set name for Container. Used for search for files in a named directory.\n
    Example: Container::new('resource') used for represent files from /resource"]
    pub fn new(name: &str) -> Container {
        let object: Container = Container {
            name: (name.to_string()),
            value: (HashMap::new()),
            root: None,
        };
        object
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.value.get(key)
    }
    pub fn get_root(&self) -> Option<OsString> {
        return self.root.clone();
    }
    pub fn for_each<F>(&self, func: F)
    where
        F: Fn(String, String),
    {
        for x in &self.value {
            func(x.0.to_string(), x.1.to_string())
        }
    }
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    pub fn load_dir(&mut self) {
        let path = &current_dir().unwrap();
        self.root.get_or_insert(OsString::from(path.as_os_str()));
        let path = path.join(&self.name);
        let files = path.read_dir();
        match &files {
            Ok(_) => println!("Used container '{}'", &self.name),
            Err(e) => {
                eprintln!("Container error: {}", e);
                return;
            }
        }

        files.unwrap().for_each(|file| {
            let _file = &file.unwrap();
            let file_name = _file.file_name().into_string().unwrap();
            let file_path = _file.path().into_os_string().into_string().unwrap();
            self.value.insert(file_name, file_path);
        });
    }
}

#[cfg(test)]
mod test {
    use crate::containers::container::Container;
    #[test] // print container 'resource'
    fn test_container_load_dir() {
        let mut container = Container::new("resource");
        container.load_dir();
        println!("{:?}", container.value)
    }
}
