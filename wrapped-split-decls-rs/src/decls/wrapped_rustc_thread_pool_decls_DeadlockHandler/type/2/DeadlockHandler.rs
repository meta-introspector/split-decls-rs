use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
# [doc = " The type for a closure that gets invoked when the Rayon thread pool deadlocks"] type DeadlockHandler = dyn Fn () + Send + Sync ;
}