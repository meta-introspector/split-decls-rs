use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " View that reverses the direction of edges in its underlying graph, so that"] # [doc = " successors become predecessors and vice-versa."] # [doc = ""] # [doc = " Because of `impl<G: Graph> Graph for &G`, the underlying graph can be"] # [doc = " wrapped by-reference instead of by-value if desired."] # [derive (Clone , Copy , Debug)] pub struct ReversedGraph < G > { pub inner : G , }