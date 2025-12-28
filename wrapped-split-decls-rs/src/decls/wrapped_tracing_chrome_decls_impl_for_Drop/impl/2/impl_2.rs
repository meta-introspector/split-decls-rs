use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl Drop for FlushGuard { fn drop (& mut self) { if let Some (handle) = self . handle . take () { let _ignored = self . sender . send (Message :: Drop) ; if handle . join () . is_err () { eprintln ! ("tracing_chrome: Trace writing thread panicked.") ; } } } }