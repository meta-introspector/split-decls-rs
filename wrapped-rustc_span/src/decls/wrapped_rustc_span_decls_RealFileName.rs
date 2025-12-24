use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Decodable, Encodable)]
pub enum RealFileName {
    LocalPath(PathBuf),
    /// For remapped paths (namely paths into libstd that have been mapped
    /// to the appropriate spot on the local host's file system, and local file
    /// system paths that have been remapped with `FilePathMapping`),
    Remapped {
        /// `local_path` is the (host-dependent) local path to the file. This is
        /// None if the file was imported from another crate
        local_path: Option<PathBuf>,
        /// `virtual_name` is the stable path rustc will store internally within
        /// build artifacts.
        virtual_name: PathBuf,
    },
}
