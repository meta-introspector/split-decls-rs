use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " The champion signature with 71 facets (factor of 71 in Monster Group)"] # [derive (Debug , Clone)] pub struct ChampionSignature { pub signature : String , pub facets : Vec < String > , pub monster_order_factor : String , }
}