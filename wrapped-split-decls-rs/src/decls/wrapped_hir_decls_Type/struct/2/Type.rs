use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , PartialEq , Eq , Debug , Hash)] pub struct Type < 'db > { env : Arc < TraitEnvironment < 'db > > , ty : Ty < 'db > , }