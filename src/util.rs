use std::{
    io::Result,
    path::{Component, PathBuf},
};

pub fn to_lexical_absolute(path: PathBuf) -> Result<PathBuf> {
    let mut absolute = if path.is_absolute() {
        std::path::PathBuf::new()
    } else {
        std::env::current_dir()?
    };    for component in path.components() {
        match component {
            Component::CurDir => {},
            Component::ParentDir => { absolute.pop(); },
            component @ _ => absolute.push(component.as_os_str()),
        }
    }
    Ok(absolute)
}
