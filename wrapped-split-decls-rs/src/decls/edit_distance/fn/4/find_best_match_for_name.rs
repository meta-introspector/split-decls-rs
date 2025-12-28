use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Finds the best match for a given word in the given iterator."] # [doc = ""] # [doc = " As a loose rule to avoid the obviously incorrect suggestions, it takes"] # [doc = " an optional limit for the maximum allowable edit distance, which defaults"] # [doc = " to one-third of the given word."] # [doc = ""] # [doc = " We use case insensitive comparison to improve accuracy on an edge case with a lower(upper)case"] # [doc = " letters mismatch."] pub fn find_best_match_for_name (candidates : & [Symbol] , lookup : Symbol , dist : Option < usize > ,) -> Option < Symbol > { find_best_match_for_name_impl (false , candidates , lookup , dist) }
}