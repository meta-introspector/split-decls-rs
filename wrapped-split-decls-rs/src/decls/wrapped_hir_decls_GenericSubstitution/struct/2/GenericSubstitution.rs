use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct GenericSubstitution < 'db > { def : GenericDefId , subst : GenericArgs < 'db > , env : Arc < TraitEnvironment < 'db > > , }
}