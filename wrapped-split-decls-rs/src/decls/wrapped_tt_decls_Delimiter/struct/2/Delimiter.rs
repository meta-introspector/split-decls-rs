use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct Delimiter < S > { pub open : S , pub close : S , pub kind : DelimiterKind , }