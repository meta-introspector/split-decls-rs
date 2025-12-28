use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [inline] fn to_sorted_vec < HCX , T , K , I > (hcx : & HCX , iter : I , cache_sort_key : bool , extract_key : fn (& T) -> & K ,) -> Vec < T > where I : Iterator < Item = T > , K : ToStableHashKey < HCX > , { let mut items : Vec < T > = iter . collect () ; if cache_sort_key { items . sort_by_cached_key (| x | extract_key (x) . to_stable_hash_key (hcx)) ; } else { items . sort_unstable_by_key (| x | extract_key (x) . to_stable_hash_key (hcx)) ; } items }
}