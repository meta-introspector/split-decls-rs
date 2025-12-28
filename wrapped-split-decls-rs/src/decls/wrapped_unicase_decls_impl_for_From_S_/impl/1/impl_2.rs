use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : AsRef < str > > From < S > for UniCase < S > { fn from (s : S) -> Self { UniCase :: new (s) } }