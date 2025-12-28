use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct TopSubtreeBuilder < S > { unclosed_subtree_indices : Vec < usize > , token_trees : Vec < TokenTree < S > > , last_closed_subtree : Option < usize > , }
}