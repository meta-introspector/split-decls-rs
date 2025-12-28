use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Clone , Copy)] pub struct SubtreeBuilderRestorePoint { unclosed_subtree_indices_len : usize , token_trees_len : usize , last_closed_subtree : Option < usize > , }