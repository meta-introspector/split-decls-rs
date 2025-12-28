use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl FileSystemOracle for DefaultFileSystemOracle { fn audit_read () -> Result < () , String > { eprintln ! ("FS_AUDIT: Read operation") ; Ok (()) } fn audit_write () -> Result < () , String > { eprintln ! ("FS_AUDIT: Write operation") ; Ok (()) } fn check_path_safety (path : & str) -> bool { ! path . contains ("..") && ! path . starts_with ("/etc") } }