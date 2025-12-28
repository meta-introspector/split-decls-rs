use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn stable_hash_hashes_as_tuple () { let hash_packed = { let mut hasher = StableHasher :: new () ; TaggedRef :: new (& 12 , Tag2 :: B11) . hash_stable (& mut () , & mut hasher) ; hasher . finish :: < Hash128 > () } ; let hash_tupled = { let mut hasher = StableHasher :: new () ; (& 12 , Tag2 :: B11) . hash_stable (& mut () , & mut hasher) ; hasher . finish :: < Hash128 > () } ; assert_eq ! (hash_packed , hash_tupled) ; }
}