use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug)] struct OwlOntology { classes : HashSet < String > , properties : HashSet < String > , individuals : HashSet < String > , triples : Vec < (String , String , String) > , domains : HashMap < String , String > , ranges : HashMap < String , String > , }