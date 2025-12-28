use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn paper_slt () { let graph = TestGraph :: new (1 , & [(1 , 2) , (1 , 3) , (2 , 3) , (2 , 7) , (3 , 4) , (3 , 6) , (4 , 5) , (5 , 4) , (6 , 7) , (7 , 8) , (8 , 5)] ,) ; dominators (& graph) ; }