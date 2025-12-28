use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Copy , Clone , PartialEq)] pub struct DelimSpan < S > { pub open : S , pub close : S , }
}