use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < S : FromStr + AsRef < str > > FromStr for UniCase < S > { type Err = < S as FromStr > :: Err ; fn from_str (s : & str) -> Result < UniCase < S > , Self :: Err > { s . parse () . map (UniCase :: new) } }