use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn each_adjacent_from_a () { let graph = create_graph () ; test_adjacent_edges (& graph , NodeIndex (0) , "A" , & [] , & [("AB" , "B")]) ; }