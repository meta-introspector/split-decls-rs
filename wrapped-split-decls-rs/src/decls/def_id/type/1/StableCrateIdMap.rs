use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdecltype! {
pub type StableCrateIdMap = indexmap :: IndexMap < StableCrateId , CrateNum , BuildHasherDefault < Unhasher > > ;
}