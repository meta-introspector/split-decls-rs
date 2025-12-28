use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [doc = " Collects code [`Suggestion`]s from one or more compiler diagnostic lines."] # [doc = ""] # [doc = " Fails if any of diagnostic line `input` is not a valid [`Diagnostic`] JSON."] # [doc = ""] # [doc = " * `only` --- only diagnostics with code in a set of error codes would be collected."] pub fn get_suggestions_from_json < S : :: std :: hash :: BuildHasher > (input : & str , only : & HashSet < String , S > , filter : Filter ,) -> serde_json :: error :: Result < Vec < Suggestion > > { let mut result = Vec :: new () ; for cargo_msg in serde_json :: Deserializer :: from_str (input) . into_iter :: < Diagnostic > () { result . extend (collect_suggestions (& cargo_msg ? , only , filter)) ; } Ok (result) }
}