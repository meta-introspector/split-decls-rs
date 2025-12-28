use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn each_adjacent_from_c () { let graph = create_graph () ; test_adjacent_edges (& graph , NodeIndex (2) , "C" , & [("EC" , "E") , ("BC" , "B")] , & []) ; }
}