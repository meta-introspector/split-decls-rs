use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ResourceVersion {
    /// Format the version as a comma-separated string of four integers
    /// as expected by Windows resource scripts for the `FILEVERSION` and `PRODUCTVERSION` fields.
    fn to_quad_string(&self) -> String {
        format!("{},{},{},{}", self.major, self.minor, self.patch, self.build)
    }
}
