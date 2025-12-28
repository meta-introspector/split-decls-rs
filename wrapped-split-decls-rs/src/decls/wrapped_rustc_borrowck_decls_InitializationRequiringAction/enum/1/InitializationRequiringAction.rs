use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Copy , Clone , Debug)] enum InitializationRequiringAction { Borrow , MatchOn , Use , Assignment , PartialAssignment , }