use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Useful type to use with `Result<>` indicate that an error has already"] # [doc = " been reported to the user, so no need to continue checking."] # [doc = ""] # [doc = " The `()` field is necessary: it is non-`pub`, which means values of this"] # [doc = " type cannot be constructed outside of this crate."] # [derive (Clone , Copy , Debug , Hash , PartialEq , Eq , PartialOrd , Ord , HashStable_Generic)] pub struct ErrorGuaranteed (()) ;