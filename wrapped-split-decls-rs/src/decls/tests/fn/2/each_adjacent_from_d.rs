use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn each_adjacent_from_d () { let graph = create_graph () ; test_adjacent_edges (& graph , NodeIndex (3) , "D" , & [("BD" , "B")] , & [("DE" , "E")]) ; }
}