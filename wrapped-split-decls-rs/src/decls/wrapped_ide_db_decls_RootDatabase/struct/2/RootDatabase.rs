use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [salsa_macros :: db] pub struct RootDatabase { storage : ManuallyDrop < salsa :: Storage < Self > > , files : Arc < Files > , crates_map : Arc < CratesMap > , nonce : Nonce , }