use serde::{Deserialize, Serialize};
use std::collections::HashMap;

fn main () -> Result < () > { println ! ("🔬 8-Level K-Theory Dependency Analysis") ; let decls = load_all_declarations () ? ; println ! ("📊 Loaded {} declarations" , decls . len ()) ; let dep_graph = build_dependency_graph (& decls) ? ; let k_analysis = analyze_k_levels (& dep_graph , & decls) ? ; report_k_theory_analysis (& k_analysis) ; Ok (()) }