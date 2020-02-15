mod analyzer;
mod synthesizer;

use crate::containers::*;

fn main() {
    const CONTAINER_NAME: &str = ".cache";
    const CONTAINER_KEY: &str = "app-conf.yml";
    let mut container = container::Container::new(CONTAINER_NAME);
    container.load_dir();
    let r = container.get(CONTAINER_KEY);
    if r.is_none() {
        panic!("qdb-conf load is not has been correctly!")
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_main() {
        main();
    }
}
