use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn single_cyclic_node () { let labels : Trivial = UnlabelledNodes (1) ; let r = test_input (LabelledGraph :: new ("single_cyclic_node" , labels , vec ! [edge (0 , 0 , "E" , Style :: None)] , None ,)) ; assert_eq ! (r . unwrap () , r#"digraph single_cyclic_node {
    N0[label="N0"];
    N0 -> N0[label="E"];
}
"#) ; }