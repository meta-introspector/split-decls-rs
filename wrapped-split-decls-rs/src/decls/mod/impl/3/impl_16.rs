use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl < O > Node < O > { fn new (parent : Option < usize > , obligation : O , obligation_tree_id : ObligationTreeId) -> Node < O > { Node { obligation , state : Cell :: new (NodeState :: Pending) , dependents : if let Some (parent_index) = parent { vec ! [parent_index] } else { vec ! [] } , has_parent : parent . is_some () , obligation_tree_id , } } }