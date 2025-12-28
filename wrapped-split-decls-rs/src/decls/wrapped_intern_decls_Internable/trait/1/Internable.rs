use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait Internable : Hash + Eq + 'static { fn storage () -> & 'static InternStorage < Self > ; }