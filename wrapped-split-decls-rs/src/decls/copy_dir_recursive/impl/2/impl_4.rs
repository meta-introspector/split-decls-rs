use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for LockFileGuard { fn drop (& mut self) { if let Err (e) = fs :: remove_file (& self . path) { eprintln ! ("Warning: Failed to remove lock file at {}: {}" , self . path . display () , e) ; } } }