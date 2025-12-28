use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " The type for a closure that gets invoked before blocking in a thread."] # [doc = " Note that this same closure may be invoked multiple times in parallel."] type ReleaseThreadHandler = dyn Fn () + Send + Sync ;