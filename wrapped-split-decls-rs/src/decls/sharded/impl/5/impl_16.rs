use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < K : Eq + Hash + Copy > ShardedHashMap < K , () > { # [inline] pub fn intern_ref < Q : ? Sized > (& self , value : & Q , make : impl FnOnce () -> K) -> K where K : Borrow < Q > , Q : Hash + Eq , { let hash = make_hash (value) ; let mut shard = self . lock_shard_by_hash (hash) ; match table_entry (& mut shard , hash , value) { Entry :: Occupied (e) => e . get () . 0 , Entry :: Vacant (e) => { let v = make () ; e . insert ((v , ())) ; v } } } # [inline] pub fn intern < Q > (& self , value : Q , make : impl FnOnce (Q) -> K) -> K where K : Borrow < Q > , Q : Hash + Eq , { let hash = make_hash (& value) ; let mut shard = self . lock_shard_by_hash (hash) ; match table_entry (& mut shard , hash , & value) { Entry :: Occupied (e) => e . get () . 0 , Entry :: Vacant (e) => { let v = make (value) ; e . insert ((v , ())) ; v } } } }
}