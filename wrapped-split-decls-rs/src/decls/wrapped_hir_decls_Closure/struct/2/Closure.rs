use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct Closure < 'db > { id : AnyClosureId , subst : GenericArgs < 'db > , }
}