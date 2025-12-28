use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < I : Idx , K , V , C > HashStable < C > for SortedIndexMultiMap < I , K , V > where K : HashStable < C > , V : HashStable < C > , { fn hash_stable (& self , ctx : & mut C , hasher : & mut StableHasher) { let SortedIndexMultiMap { items , idx_sorted_by_item_key : _ , } = self ; items . hash_stable (ctx , hasher) } }