use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a , O : ForestObligation + 'a > dot :: Labeller < 'a > for & 'a ObligationForest < O > { type Node = usize ; type Edge = (usize , usize) ; fn graph_id (& self) -> dot :: Id < '_ > { dot :: Id :: new ("trait_obligation_forest") . unwrap () } fn node_id (& self , index : & Self :: Node) -> dot :: Id < '_ > { dot :: Id :: new (format ! ("obligation_{index}")) . unwrap () } fn node_label (& self , index : & Self :: Node) -> dot :: LabelText < '_ > { let node = & self . nodes [* index] ; let label = format ! ("{:?} ({:?})" , node . obligation . as_cache_key () , node . state . get ()) ; dot :: LabelText :: LabelStr (label . into ()) } fn edge_label (& self , (_index_source , _index_target) : & Self :: Edge) -> dot :: LabelText < '_ > { dot :: LabelText :: LabelStr ("" . into ()) } }
}