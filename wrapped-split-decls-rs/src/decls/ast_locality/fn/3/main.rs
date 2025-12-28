use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () -> Result < () > { println ! ("🧠 AST Locality Analyzer - 99% Local / 1% Global") ; let decls = load_all_declarations () ? ; println ! ("📊 Loaded {} AST nodes" , decls . len ()) ; let locality_analysis = analyze_locality (& decls) ? ; let sparse_matrix = create_sparse_matrix (& decls , & locality_analysis) ? ; report_locality_findings (& locality_analysis , & sparse_matrix) ; Ok (()) }