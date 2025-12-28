use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : AsRef < str > > Eq for UniCase < S > { }