use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn create_graph () -> TestGraph { let mut graph = LinkedGraph :: new () ; let a = graph . add_node ("A") ; let b = graph . add_node ("B") ; let c = graph . add_node ("C") ; let d = graph . add_node ("D") ; let e = graph . add_node ("E") ; let f = graph . add_node ("F") ; graph . add_edge (a , b , "AB") ; graph . add_edge (b , c , "BC") ; graph . add_edge (b , d , "BD") ; graph . add_edge (d , e , "DE") ; graph . add_edge (e , c , "EC") ; graph . add_edge (f , b , "FB") ; return graph ; }