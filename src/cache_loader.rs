use crate::containers::container::Container;
use std::fs;
use std::fs::create_dir;
use std::path::PathBuf;

const CONTAINER_NAME: &str = "resource";
const CONTAINER_NAME_CACHE: &str = ".cache";

fn is_cache_empty() -> bool {
    let mut container = Container::new(CONTAINER_NAME_CACHE);
    container.load_dir();
    container.is_empty()
}

fn create_cache() -> bool {
    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();
    let root = container.get_root();
    assert_eq!(root.is_some(), true);
    let path = PathBuf::from(root.unwrap());
    match create_dir(path.join(".cache")) {
        Ok(_) => true,
        Err(e) => {
            println!("Error: {:?}", e);
            false
        }
    }
}
//copy files from '*/resource/' folder to '*/resource/.cache/'
fn copy_from_resource_to_cache() -> bool {
    let mut container = Container::new(CONTAINER_NAME);
    container.load_dir();

    let mut container_cache = Container::new(CONTAINER_NAME_CACHE);
    container_cache.load_dir();

    let path = container_cache.get_root().unwrap();
    let cache_word = String::from(CONTAINER_NAME_CACHE);
    container.for_each(|key, val| {
        if key != cache_word {
            let from = PathBuf::from(&val);
            let from = from.as_path();
            let to = PathBuf::from(&path).join(CONTAINER_NAME_CACHE).join(key);
            let to = to.as_path();
            let _r = fs::copy(from, to);
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

    #[test]
    fn test_exec() {
        if is_cache_empty() {
            assert_eq!(create_cache(), true, "cache folder not created");
        }
        copy_from_resource_to_cache();
        assert_eq!(
            is_cache_empty(),
            false,
            "cache folder is empty or not found"
        );
    }
}
