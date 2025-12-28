use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclenum! {
# [doc = " The style for a node or edge."] # [doc = " See <https://www.graphviz.org/docs/attr-types/style/> for descriptions."] # [doc = " Note that some of these are not valid for edges."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] pub enum Style { None , Solid , Dashed , Dotted , Bold , Rounded , Diagonals , Filled , Striped , Wedged , }
}