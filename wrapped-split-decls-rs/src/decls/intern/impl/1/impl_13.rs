use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T > Eq for Interned < 'a , T > { }