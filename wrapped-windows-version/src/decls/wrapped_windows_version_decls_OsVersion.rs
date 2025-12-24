use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Operating system version information.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OsVersion {
    /// The major version number of the operating system.
    pub major: u32,
    /// The minor version number of the operating system.
    pub minor: u32,
    /// The major version number of the latest service pack installed on the system.
    pub pack: u32,
    /// The build number of the operating system.
    pub build: u32,
}
