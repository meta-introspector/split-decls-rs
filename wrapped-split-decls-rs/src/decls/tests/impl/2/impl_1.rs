use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < T : Eq + Hash + Copy > TransitiveRelation < T > { # [doc = " A \"best\" parent in some sense. See `parents` and"] # [doc = " `postdom_upper_bound` for more details."] fn postdom_parent (& self , a : T) -> Option < T > { self . mutual_immediate_postdominator (self . parents (a)) } }
}