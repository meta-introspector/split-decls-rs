use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn generate_example_probes (output : & str) -> Result < () > { let probes = create_example_probes () ; let json = serde_json :: to_string_pretty (& probes) ? ; std :: fs :: write (output , json) ? ; println ! ("📋 Generated example probes configuration: {}" , output) ; println ! ("🔧 Edit this file to customize your AST probes") ; println ! ("\n📖 Generated Probes:") ; for probe in & probes { println ! ("  🔍 {}: {:?} -> {:?}" , probe . name , probe . node_type , probe . action) ; } Ok (()) }