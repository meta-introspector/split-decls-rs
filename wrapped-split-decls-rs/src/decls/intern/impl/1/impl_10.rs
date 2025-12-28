use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < 'a , T > Copy for Interned < 'a , T > { }