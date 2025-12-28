use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn apply_sparql_probes (dir : & str , probes_file : & str , verbose : bool) -> Result < () > { println ! ("🚀 Applying SPARQL-generated probes to directory: {}" , dir) ; let mut reflector = AstReflector :: new () ; reflector . load_probes (Path :: new (probes_file)) ? ; if verbose { println ! ("📋 Loaded probes from: {}" , probes_file) ; } reflector . reflect_directory (Path :: new (dir)) ? ; println ! ("✅ SPARQL probe application completed!") ; Ok (()) }