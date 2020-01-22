use crate::containers::container::Container;
use std::fs;
use std::fs::{create_dir, File};
use std::path::PathBuf;

fn is_cache_empty() -> bool {
    const CONTAINER_NAME: &str = "resource";

    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();

    match container.get(".cache") {
        Some(_) => false,
        None => true,
    }
}

fn create_cache() -> bool {
    const CONTAINER_NAME: &str = "resource";

    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();
    let root = container.get_root();
    assert_eq!(root.is_some(), true);
    let path = PathBuf::from(root.unwrap());
    match create_dir(path.join(CONTAINER_NAME).join(".cache")) {
        Ok(_) => true,
        Err(e) => {
            println!("Error: {}", e);
            false
        }
    }
}
//copy files from '*/resource/' folder to '*/resource/.cache/'
fn copy_from_resource_to_cache() -> bool {
    const CONTAINER_NAME: &str = "resource";

    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();
    let path = container.get(".cache").unwrap();
    let root = container.get_root().unwrap();
    let cache_word = String::from(".cache");
    container.for_each(|key, val| {
        if key != cache_word {
            let target_path = PathBuf::from(path).join(key);
            let target_path = target_path.as_path();
            let r = fs::copy(val, target_path);
            match r {
                Ok(f) => println!("res: {}", f),
                Err(e) => println!("err: {}", e),
            }
        }
    });
    true
}

#[doc = "(is_cache_empty -> create_cache) && copy_from_resource_to_cache"]
pub fn exec() {
    if is_cache_empty() {
        create_cache();
    }
    copy_from_resource_to_cache();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test] //container 'resource' check file '.cache'
    fn test_is_cache_empty() {
        assert_eq!(is_cache_empty(), false, "Cache folder is not found");
    }

    #[test]
    fn test_create_cache() {
        create_cache();
    }

    #[test]
    fn test_get_from_resource() {
        get_from_resource();
    }
}
