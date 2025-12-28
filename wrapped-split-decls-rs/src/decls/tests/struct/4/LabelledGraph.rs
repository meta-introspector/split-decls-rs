use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct LabelledGraph { # [doc = " The name for this graph. Used for labeling generated `digraph`."] name : & 'static str , # [doc = " Each node is an index into `node_labels`; these labels are"] # [doc = " used as the label text for each node. (The node *names*,"] # [doc = " which are unique identifiers, are derived from their index"] # [doc = " in this array.)"] # [doc = ""] # [doc = " If a node maps to None here, then just use its name as its"] # [doc = " text."] node_labels : Vec < Option < & 'static str > > , node_styles : Vec < Style > , # [doc = " Each edge relates a from-index to a to-index along with a"] # [doc = " label; `edges` collects them."] edges : Vec < Edge > , }
}