use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < G > TriColorVisitor < G > for CycleDetector where G : ? Sized + DirectedGraph , { type BreakVal = () ; fn node_examined (& mut self , _node : G :: Node , prior_status : Option < NodeStatus > ,) -> ControlFlow < Self :: BreakVal > { match prior_status { Some (NodeStatus :: Visited) => ControlFlow :: Break (()) , _ => ControlFlow :: Continue (()) , } } }
}