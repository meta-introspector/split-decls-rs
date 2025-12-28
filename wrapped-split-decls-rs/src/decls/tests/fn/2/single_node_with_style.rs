use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [test] fn single_node_with_style () { let labels : Trivial = UnlabelledNodes (1) ; let styles = Some (vec ! [Style :: Dashed]) ; let r = test_input (LabelledGraph :: new ("single_node" , labels , vec ! [] , styles)) ; assert_eq ! (r . unwrap () , r#"digraph single_node {
    N0[label="N0"][style="dashed"];
}
"#) ; }