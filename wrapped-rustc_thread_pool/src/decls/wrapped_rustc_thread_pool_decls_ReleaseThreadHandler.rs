use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The type for a closure that gets invoked before blocking in a thread.
/// Note that this same closure may be invoked multiple times in parallel.
type ReleaseThreadHandler = dyn Fn() + Send + Sync;
