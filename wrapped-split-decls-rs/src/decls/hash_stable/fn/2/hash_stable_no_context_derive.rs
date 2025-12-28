use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub (crate) fn hash_stable_no_context_derive (s : synstructure :: Structure < '_ > ,) -> proc_macro2 :: TokenStream { hash_stable_derive_with_mode (s , HashStableMode :: NoContext) }