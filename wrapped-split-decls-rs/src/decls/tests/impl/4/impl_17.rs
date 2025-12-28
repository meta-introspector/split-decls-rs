use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl < 'a > Labeller < 'a > for LabelledGraphWithEscStrs { type Node = Node ; type Edge = & 'a Edge ; fn graph_id (& 'a self) -> Id < 'a > { self . graph . graph_id () } fn node_id (& 'a self , n : & Node) -> Id < 'a > { self . graph . node_id (n) } fn node_label (& 'a self , n : & Node) -> LabelText < 'a > { match self . graph . node_label (n) { LabelStr (s) | EscStr (s) | HtmlStr (s) => EscStr (s) , } } fn edge_label (& 'a self , e : & & 'a Edge) -> LabelText < 'a > { match self . graph . edge_label (e) { LabelStr (s) | EscStr (s) | HtmlStr (s) => EscStr (s) , } } }
}