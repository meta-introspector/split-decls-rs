use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum FileNameEmbeddablePreference {
    /// If a remapped path is available, only embed the `virtual_path` and omit the `local_path`.
    ///
    /// Otherwise embed the local-path into the `virtual_path`.
    RemappedOnly,
    /// Embed the original path as well as its remapped `virtual_path` component if available.
    LocalAndRemapped,
}
