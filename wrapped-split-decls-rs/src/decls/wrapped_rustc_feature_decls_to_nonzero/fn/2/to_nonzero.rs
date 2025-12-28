use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const fn to_nonzero (n : Option < u32 >) -> Option < NonZero < u32 > > { match n { None => None , Some (n) => NonZero :: new (n) , } }