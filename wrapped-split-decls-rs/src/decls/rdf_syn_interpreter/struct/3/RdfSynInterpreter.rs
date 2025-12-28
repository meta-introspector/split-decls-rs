use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " RDF-backed interpreter that executes extracted split-decls-rs main routine step by step"] pub struct RdfSynInterpreter { # [doc = " Loaded function declarations from split-decls-rs"] functions : HashMap < String , ItemFn > , # [doc = " RDF triples representing execution state"] rdf_state : Vec < (String , String , String) > , # [doc = " Current execution context"] execution_stack : Vec < ExecutionFrame > , # [doc = " Step counter for debugging"] step_count : usize , }