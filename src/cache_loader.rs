use crate::containers::container::Container;
use std::fs::create_dir;
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

fn get_from_resource() {
    const CONTAINER_NAME: &str = "resource";

    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();
    if container.is_empty() {
        println!("Container '{}' empty", CONTAINER_NAME)
    }
    container.for_each(|x, y| println!("key: {} | value: {}", x, y));
}

#[doc = "(is_cache_empty -> create_cache) && get_from_resource"]
pub fn exec() {
    if is_cache_empty() {
        create_cache();
    }
    get_from_resource();
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
