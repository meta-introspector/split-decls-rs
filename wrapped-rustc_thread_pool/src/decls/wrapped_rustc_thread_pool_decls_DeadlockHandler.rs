use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The type for a closure that gets invoked when the Rayon thread pool deadlocks
type DeadlockHandler = dyn Fn() + Send + Sync;
