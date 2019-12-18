use std::collections::{HashSet, HashMap};
use std::collections::hash_map::RandomState;
use std::path::{Path, PathBuf};
use std::env::current_dir;

// Container is instance for get/set values for safety access data
pub struct Container {
   pub name:    String,
       value:    HashMap<String,RandomState>
}

// Implementation for container-based instance
impl Container {
pub fn new(name : &str) -> Container{
    let object : Container = Container {
        name: (name.to_string()),
        value: (HashMap::new())
    };
    object
}
pub fn get  ( &mut self, key : &str) -> Option<&RandomState>  {
   self.value.get(key)
}
pub fn set ( &mut self, key: String, value: RandomState) -> Option<RandomState> {
    self.value.insert(key,value)
}
pub fn load_dir (&self) {
    let path = &current_dir().unwrap();
    let path = path.join("src").join(&self.name);
    assert_eq!(path.as_os_str(), "C:\\Users\\user\\Documents\\RustProjects\\qdb\\resource")
}
}

#[cfg(test)]
mod test{
    use crate::containers::container::Container;
    use std::env::current_dir;

    #[test]
    fn test_container_load_dir(){
        let container = Container::new("resource");
        let path = &current_dir().unwrap();
        let path = path.join("resource");
        assert_eq!(path.as_os_str(), "C:\\Users\\user\\Documents\\RustProjects\\qdb\\resource")
    }
}