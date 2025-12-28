use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A unique ID associated with a macro invocation and expansion."] # [derive (Clone , Copy , PartialEq , Eq , Hash)] pub struct ExpnId { pub krate : CrateNum , pub local_id : ExpnIndex , }
}