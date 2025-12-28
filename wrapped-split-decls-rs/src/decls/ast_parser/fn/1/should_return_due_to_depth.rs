use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn should_return_due_to_depth (depth : usize , max_depth : usize) -> bool { depth > max_depth }