use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Clone)] pub struct MonsterSignature { pub original : String , pub pairs : Vec < String > , pub triples : Vec < String > , pub pentas : Vec < String > , pub heptas : Vec < String > , pub compressed_form : String , }