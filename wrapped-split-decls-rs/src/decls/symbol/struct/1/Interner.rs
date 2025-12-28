use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (crate) struct Interner (Lock < InternerInner >) ;