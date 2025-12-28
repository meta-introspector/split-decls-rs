use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] struct SparqlEngine { graph : Graph < String , String , Directed > , node_map : HashMap < String , NodeIndex > , triples : Vec < (String , String , String) > , }