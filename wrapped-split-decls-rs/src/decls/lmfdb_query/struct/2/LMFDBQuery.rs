use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug)] pub struct LMFDBQuery { pub collection : String , pub query_params : HashMap < String , Value > , pub similarity_features : Vec < String > , }
}