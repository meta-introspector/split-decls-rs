use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn each_adjacent_from_b () { let graph = create_graph () ; test_adjacent_edges (& graph , NodeIndex (1) , "B" , & [("FB" , "F") , ("AB" , "A")] , & [("BD" , "D") , ("BC" , "C")] ,) ; }
}