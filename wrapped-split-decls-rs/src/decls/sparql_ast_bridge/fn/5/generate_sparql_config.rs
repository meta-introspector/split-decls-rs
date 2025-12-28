use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: generate_sparql_config");
fn generate_sparql_config (output : & str) -> Result < () > { let config = create_example_sparql_config () ; let json = serde_json :: to_string_pretty (& config) ? ; std :: fs :: write (output , json) ? ; println ! ("📋 Generated SPARQL configuration: {}" , output) ; println ! ("🔧 Edit this file to customize SPARQL queries and probe templates") ; println ! ("\n📖 Generated queries:") ; for query in & config . queries { println ! ("  🔍 {}: {:?}" , query . name , query . query_type) ; } println ! ("\n⚡ Generated templates:") ; for (name , template) in & config . probe_templates { println ! ("  📝 {}: {:?} -> {:?}" , name , template . node_type , template . wrapper_function) ; } Ok (()) }
}