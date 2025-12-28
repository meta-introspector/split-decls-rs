use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type StableCrateIdMap = indexmap :: IndexMap < StableCrateId , CrateNum , BuildHasherDefault < Unhasher > > ;