use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct RdfInterpreter { triples : Vec < (String , String , String) > , predicates : HashMap < String , usize > , subjects : HashMap < String , usize > , objects : HashMap < String , usize > , state : RdfState , }
}