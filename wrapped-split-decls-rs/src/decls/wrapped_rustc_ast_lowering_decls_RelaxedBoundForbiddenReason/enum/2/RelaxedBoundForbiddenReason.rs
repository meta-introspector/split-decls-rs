use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [derive (Clone , Copy , Debug)] enum RelaxedBoundForbiddenReason { TraitObjectTy , SuperTrait , TraitAlias , AssocTyBounds , LateBoundVarsInScope , }
}