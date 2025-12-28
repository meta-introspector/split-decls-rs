use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait IntoPointer { # [doc = " Returns a pointer which outlives `self`."] fn into_pointer (& self) -> * const () ; }