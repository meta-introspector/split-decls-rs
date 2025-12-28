use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
# [test] fn single_node () { let labels : Trivial = UnlabelledNodes (1) ; let r = test_input (LabelledGraph :: new ("single_node" , labels , vec ! [] , None)) ; assert_eq ! (r . unwrap () , r#"digraph single_node {
    N0[label="N0"];
}
"#) ; }
}