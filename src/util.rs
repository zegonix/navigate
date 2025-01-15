use std::{
    io::Result,
    path::{Component, PathBuf},
};

/// returns absolute path but keeps links
pub fn to_rooted(path: &mut PathBuf) -> Result<()> {
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
    *path = absolute;
    Ok(())
}

