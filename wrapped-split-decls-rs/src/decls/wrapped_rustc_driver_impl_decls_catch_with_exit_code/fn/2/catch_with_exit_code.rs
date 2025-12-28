use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Variant of `catch_fatal_errors` for the `interface::Result` return type"] # [doc = " that also computes the exit code."] pub fn catch_with_exit_code (f : impl FnOnce ()) -> i32 { match catch_fatal_errors (f) { Ok (()) => EXIT_SUCCESS , _ => EXIT_FAILURE , } }