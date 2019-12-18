use std::fs::File;      // Implement input / output container for resource folder.
use std::path::Path;
use crate::containers::container::Container;

// {Container} ∈ IoContainerResource
pub struct IoContainerResource {
     value : Container
}

impl IoContainerResource {

pub fn new() -> IoContainerResource {
    let object : IoContainerResource = IoContainerResource{ value: (Container::new("resource")) };
    object
}

}
