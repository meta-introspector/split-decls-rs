use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Serialize , Deserialize)] pub struct BootstrapTrace { pub states : HashMap < String , ExecutionState > , pub transitions : Vec < (String , String) > , pub rdf_graph : Vec < RdfTriple > , }
}