use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < P , T : Tag > Eq for TaggedRef < '_ , P , T > { }