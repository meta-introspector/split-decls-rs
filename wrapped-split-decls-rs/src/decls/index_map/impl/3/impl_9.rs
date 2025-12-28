use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < I : Idx , K : Ord , V > FromIterator < (K , V) > for SortedIndexMultiMap < I , K , V > { fn from_iter < J > (iter : J) -> Self where J : IntoIterator < Item = (K , V) > , { let items = IndexVec :: < I , _ > :: from_iter (iter) ; let mut idx_sorted_by_item_key : Vec < _ > = items . indices () . collect () ; idx_sorted_by_item_key . sort_by_key (| & idx | & items [idx] . 0) ; SortedIndexMultiMap { items , idx_sorted_by_item_key } } }
}