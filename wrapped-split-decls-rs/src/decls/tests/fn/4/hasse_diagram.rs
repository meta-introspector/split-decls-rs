use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn hasse_diagram () { let labels = AllNodesLabelled (vec ! ["{x,y}" , "{x}" , "{y}" , "{}"]) ; let r = test_input (LabelledGraph :: new ("hasse_diagram" , labels , vec ! [edge (0 , 1 , "" , Style :: None) , edge (0 , 2 , "" , Style :: None) , edge (1 , 3 , "" , Style :: None) , edge (2 , 3 , "" , Style :: None) ,] , None ,)) ; assert_eq ! (r . unwrap () , r#"digraph hasse_diagram {
    N0[label="{x,y}"];
    N1[label="{x}"];
    N2[label="{y}"];
    N3[label="{}"];
    N0 -> N1[label=""];
    N0 -> N2[label=""];
    N1 -> N3[label=""];
    N2 -> N3[label=""];
}
"#) ; }