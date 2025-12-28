use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for AbortIfPanic { fn drop (& mut self) { eprintln ! ("Rayon: detected unexpected panic; aborting") ; :: std :: process :: abort () ; } }