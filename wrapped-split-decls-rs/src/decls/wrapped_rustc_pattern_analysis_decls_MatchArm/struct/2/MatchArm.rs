use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The arm of a match expression."] # [derive (Debug)] pub struct MatchArm < 'p , Cx : PatCx > { pub pat : & 'p DeconstructedPat < Cx > , pub has_guard : bool , pub arm_data : Cx :: ArmData , }
}