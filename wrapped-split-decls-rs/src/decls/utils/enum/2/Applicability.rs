use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " `Applicability` of a suggestion - mirrors `rustc_errors::Applicability` - and used to represent"] # [doc = " the user's selection of applicability if specified in an attribute."] # [derive (Clone , Copy)] pub (crate) enum Applicability { MachineApplicable , MaybeIncorrect , HasPlaceholders , Unspecified , }