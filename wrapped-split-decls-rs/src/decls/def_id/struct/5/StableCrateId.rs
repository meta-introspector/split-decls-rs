use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " A [`StableCrateId`] is a 64-bit hash of a crate name, together with all"] # [doc = " `-Cmetadata` arguments, and some other data. It is to [`CrateNum`] what [`DefPathHash`] is to"] # [doc = " [`DefId`]. It is stable across compilation sessions."] # [doc = ""] # [doc = " Since the ID is a hash value, there is a small chance that two crates"] # [doc = " end up with the same [`StableCrateId`]. The compiler will check for such"] # [doc = " collisions when loading crates and abort compilation in order to avoid"] # [doc = " further trouble."] # [doc = ""] # [doc = " For more information on the possibility of hash collisions in rustc,"] # [doc = " see the discussion in [`DefId`]."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Debug)] # [derive (Hash , HashStable_Generic , Encodable , Decodable)] pub struct StableCrateId (pub (crate) Hash64) ;
}