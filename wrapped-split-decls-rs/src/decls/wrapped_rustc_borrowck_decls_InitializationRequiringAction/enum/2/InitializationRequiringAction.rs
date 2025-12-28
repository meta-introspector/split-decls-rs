use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Copy , Clone , Debug)] enum InitializationRequiringAction { Borrow , MatchOn , Use , Assignment , PartialAssignment , }
}