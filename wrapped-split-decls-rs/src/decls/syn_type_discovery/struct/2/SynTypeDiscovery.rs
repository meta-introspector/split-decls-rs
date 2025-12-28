use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default)] pub struct SynTypeDiscovery { pub discovered_types : HashSet < String > , pub visit_methods : HashSet < String > , pub enum_variants : HashSet < String > , }
}