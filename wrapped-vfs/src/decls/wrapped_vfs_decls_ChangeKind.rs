use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Kind of [file change](ChangedFile).
#[derive(Eq, PartialEq, Debug)]
pub enum ChangeKind {
    /// The file was (re-)created
    Create,
    /// The file was modified
    Modify,
    /// The file was deleted
    Delete,
}
