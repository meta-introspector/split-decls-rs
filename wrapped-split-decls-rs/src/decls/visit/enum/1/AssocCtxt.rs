use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug , PartialEq)] pub enum AssocCtxt { Trait , Impl { of_trait : bool } , }