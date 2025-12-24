use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Is `path` a symlink to a parent directory?
///
/// Including this path is guaranteed to cause an infinite loop. This
/// heuristic is not sufficient to catch all symlink cycles (it's
/// possible to construct cycle using two or more symlinks), but it
/// catches common cases.
fn path_might_be_cyclic(path: &Path) -> bool {
    let Ok(destination) = std::fs::read_link(path) else {
        return false;
    };
    let is_relative_parent = destination
        .components()
        .all(|c| matches!(c, Component::CurDir | Component::ParentDir));
    is_relative_parent || path.starts_with(destination)
}
