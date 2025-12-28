use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < HCX , T1 : ToStableHashKey < HCX > , T2 : ToStableHashKey < HCX > > ToStableHashKey < HCX > for (T1 , T2) { type KeyType = (T1 :: KeyType , T2 :: KeyType) ; # [inline] fn to_stable_hash_key (& self , hcx : & HCX) -> Self :: KeyType { (self . 0 . to_stable_hash_key (hcx) , self . 1 . to_stable_hash_key (hcx)) } }