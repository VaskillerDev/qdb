use std::collections::HashMap;
use std::env::current_dir;

// Container is instance for get/set values for safety access data
pub struct Container {
    name: String,
    value: HashMap<String, String>,
}

// Implementation for container-based instance
impl Container {
    pub fn new(name: &str) -> Container {
        let object: Container = Container {
            name: (name.to_string()),
            value: (HashMap::new()),
        };
        object
    }
    pub fn get(&self, key: &str) -> Option<&String> {
        self.value.get(key)
    }
    pub fn load_dir(&mut self) {
        let path = &current_dir().unwrap();
        let path = path.join(&self.name);
        let files = path.read_dir();
        match &files {
            Ok(_) => println!("Container '{}' has been find", &self.name),
            Err(e) => println!("{}", e),
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

    #[test]
    fn test_container_load_dir() {
        let mut container = Container::new("resource");
        container.load_dir();
        println!("{:?}", container.value)
    }
}
