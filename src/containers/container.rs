use std::collections::{HashSet, HashMap};
use std::collections::hash_map::RandomState;

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

}